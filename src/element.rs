use component::Component;
use reconciler::NodeCreator;
use std::marker::PhantomData;

pub trait HostElement: 'static + Sized + Clone {
    type DomNode;

    fn new_dom_node(h: Self, children: Vec<Self::DomNode>) -> Self::DomNode;
}

pub enum Element<H: HostElement> {
    Host {
        element: H,
        children: Vec<Element<H>>,
    },
    Stateful(Box<dyn NodeCreator<H>>),
}

pub struct StatefulElement<
    H: HostElement,
    Class: Component<H>,
> {
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

    pub fn new_functional<F, Props>(_func: F, _props: Props, _children: Vec<Element<H>>) -> Element<H>
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
