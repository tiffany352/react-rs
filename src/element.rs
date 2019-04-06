use component::Component;
use reconciler::StatefulElementWrapper;
use std::marker::PhantomData;

pub trait DomNode<'a>
where
    Self: 'a + Sized,
{
    type Widget;

    fn new_dom_node(h: &'a Self::Widget, children: Vec<Self>) -> Self;
}

pub trait HostElement: 'static + Sized {}

pub enum Element<H: HostElement> {
    Host {
        element: H,
        children: Vec<Element<H>>,
    },
    Stateful(Box<dyn StatefulElementWrapper<H>>),
}

pub struct StatefulElement<H: HostElement, Class: Component<H>> {
    pub props: Class::Props,
    _phantom: PhantomData<(H, Class)>,
}

impl<H: HostElement> Element<H> {
    pub fn new_host(elt: H, children: Vec<Element<H>>) -> Element<H> {
        Element::Host {
            element: elt,
            children: children,
        }
    }

    pub fn new_functional<F, Props>(
        _func: F,
        _props: Props,
        _children: Vec<Element<H>>,
    ) -> Element<H>
    where
        F: Fn(&Props, &[Element<H>]) -> Element<H> + 'static,
        Props: 'static,
    {
        unimplemented!()
    }

    pub fn new_stateful<Class>(props: Class::Props) -> Element<H>
    where
        Class: Component<H> + 'static,
    {
        Element::Stateful(Box::new(StatefulElement {
            props: props,
            _phantom: PhantomData::<(H, Class)>,
        }))
    }
}
