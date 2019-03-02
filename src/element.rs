use component::Component;

pub type ElementBox = Box<dyn Element>;

pub trait Element {
    fn to_host(&self) -> Option<&HostElement> { None }

    fn render(&self) -> ElementBox;
}

pub struct DivProps {
    pub children: Vec<ElementBox>,
}

impl DivProps {
    pub fn new() -> DivProps {
        DivProps {
            children: vec![],
        }
    }
}

pub enum HostElement {
    Div(DivProps),
}

impl Element for HostElement {
    fn to_host(&self) -> Option<&HostElement> { Some(self) }

    fn render(&self) -> ElementBox {
        unimplemented!()
    }
}

struct FunctionalElement<F: Fn(&Props) -> ElementBox, Props = ()> {
    func: F,
    props: Props,
}

impl<F: Fn(&Props) -> ElementBox, Props> Element for FunctionalElement<F, Props> {
    fn render(&self) -> ElementBox {
        (self.func)(&self.props)
    }
}

struct StatefulElement<Class: Component<Props, State>, Props = (), State = ()> {
    component: Class,
    props: Props,
    state: State,
}

impl<Class: Component<Props, State>, Props, State> Element for StatefulElement<Class, Props, State> {
    fn render(&self) -> ElementBox {
        self.component.render(&self.props, &self.state)
    }
}

impl Element {
    pub fn new_host(elt: HostElement) -> ElementBox {
        Box::new(elt) as ElementBox
    }

    pub fn new_functional<F, Props>(func: F, props: Props) -> ElementBox 
    where F: Fn(&Props) -> ElementBox + 'static, Props: 'static {
        Box::new(FunctionalElement {
            func: func,
            props: props,
        }) as ElementBox
    }

    pub fn new_stateful<Class, Props, State>(component: Class, props: Props) -> ElementBox
    where Class: Component<Props, State> + 'static, Props: 'static, State: 'static + Default {
        Box::new(StatefulElement {
            component: component,
            props: props,
            state: Default::default(),
        }) as ElementBox
    }
}