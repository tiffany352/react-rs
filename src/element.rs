use component::Component;
use reconciler::StatefulElementWrapper;
use std::marker::PhantomData;

pub trait HostElement: 'static + Sized + Clone {
    type DomNode;

    fn new_dom_node(h: Self, children: Vec<Self::DomNode>) -> Self::DomNode;
}

#[derive(Clone)]
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

impl<H, Class> Clone for StatefulElement<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn clone(&self) -> Self {
        StatefulElement {
            props: self.props.clone(),
            _phantom: PhantomData,
        }
    }
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
