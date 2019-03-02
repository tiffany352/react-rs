use component::Component;
use std::marker::PhantomData;

pub trait HostElement: 'static + Sized {
    type VirtualNode;

    fn new_virtual_node(h: Self, children: Vec<Self::VirtualNode>) -> Self::VirtualNode;
}

trait Renderable<H: HostElement> {
    fn render(&self, children: &[Element<H>]) -> Element<H>;
}

enum ElementType<H: HostElement> {
    Host(H),
    Renderable(Box<dyn Renderable<H>>),
}

pub struct Element<H: HostElement> {
    ty: ElementType<H>,
    children: Vec<Element<H>>,
}

struct FunctionalElement<H, F, Props = ()>
where
    H: HostElement,
    F: Fn(&Props, &[Element<H>]) -> Element<H>,
{
    func: F,
    props: Props,
    _phantom: PhantomData<H>,
}

impl<H: HostElement, F: Fn(&Props, &[Element<H>]) -> Element<H>, Props> Renderable<H>
    for FunctionalElement<H, F, Props>
{
    fn render(&self, children: &[Element<H>]) -> Element<H> {
        (self.func)(&self.props, children)
    }
}

struct StatefulElement<H: HostElement, Class: Component<H, Props, State>, Props = (), State = ()> {
    component: Class,
    props: Props,
    state: State,
    _phantom: PhantomData<H>,
}

impl<H: HostElement, Class: Component<H, Props, State>, Props, State> Renderable<H>
    for StatefulElement<H, Class, Props, State>
{
    fn render(&self, children: &[Element<H>]) -> Element<H> {
        self.component.render(&self.props, &self.state, children)
    }
}

impl<H: HostElement> Element<H> {
    pub fn new_host(elt: H, children: Vec<Element<H>>) -> Element<H> {
        Element {
            ty: ElementType::Host(elt),
            children: children,
        }
    }

    pub fn new_functional<F, Props>(func: F, props: Props, children: Vec<Element<H>>) -> Element<H>
    where
        F: Fn(&Props, &[Element<H>]) -> Element<H> + 'static,
        Props: 'static,
    {
        Element {
            ty: ElementType::Renderable(Box::new(FunctionalElement {
                func: func,
                props: props,
                _phantom: PhantomData,
            }) as Box<Renderable<H>>),
            children: children,
        }
    }

    pub fn new_stateful<Class, Props, State>(
        component: Class,
        props: Props,
        children: Vec<Element<H>>,
    ) -> Element<H>
    where
        Class: Component<H, Props, State> + 'static,
        Props: 'static,
        State: 'static + Default,
    {
        Element {
            ty: ElementType::Renderable(Box::new(StatefulElement {
                component: component,
                props: props,
                state: Default::default(),
                _phantom: PhantomData,
            }) as Box<Renderable<H>>),
            children: children,
        }
    }

    pub fn render(self) -> Element<H> {
        match self.ty {
            ElementType::Host(_) => self,
            ElementType::Renderable(elt) => elt.render(&self.children[..]),
        }
    }

    pub fn reify(self) -> H::VirtualNode {
        match self.ty {
            ElementType::Host(elt) => H::new_virtual_node(
                elt,
                self.children
                    .into_iter()
                    .map(|c| c.reify())
                    .collect::<Vec<_>>(),
            ),
            ElementType::Renderable(elt) => elt.render(&self.children[..]).reify(),
        }
    }
}
