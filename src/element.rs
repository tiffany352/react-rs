use component::Component;
use reconciler::StatefulElementWrapper;
use toolshed::Arena;

pub trait DomNode<'a>
where
    Self: 'a + Sized,
{
    type Widget;

    fn new_dom_node(h: &'a Self::Widget, children: Vec<Self>) -> Self;
}

pub trait HostElement: Sized + Clone + Copy {}

#[derive(Copy, Clone)]
pub enum Element<'arena, H: HostElement> {
    Host {
        element: H,
        children: &'arena [&'arena Element<'arena, H>],
    },
    Stateful(&'arena dyn StatefulElementWrapper<H>),
}

pub struct StatefulElement<H: HostElement, Class: Component<H>> {
    pub props: Class::Props,
}

impl<H, Class> Clone for StatefulElement<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<H, Class> Copy for StatefulElement<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
}

impl<'arena, H> Element<'arena, H>
where
    H: HostElement + 'arena,
{
    pub fn new_host(
        arena: &'arena Arena,
        elt: H,
        children: &[&'arena Element<'arena, H>],
    ) -> &'arena Element<'arena, H> {
        arena.alloc(Element::Host {
            element: elt,
            children: arena.alloc_slice(children),
        })
    }

    pub fn new_stateful<Class>(
        arena: &'arena Arena,
        props: Class::Props,
    ) -> &'arena Element<'arena, H>
    where
        Class: Component<H> + 'arena,
        Class::Props: 'arena,
    {
        let element: &'arena StatefulElement<H, Class> =
            arena.alloc(StatefulElement { props: props });
        arena.alloc(Element::Stateful(element))
    }
}
