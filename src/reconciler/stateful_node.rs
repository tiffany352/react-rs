use component::Component;
use element::{HostElement, StatefulElement};
use reconciler::NodeCreator;
use reconciler::VirtualNode;
use std::marker::PhantomData;

struct StatefulNode<H, Class, Props, State>
where
    H: HostElement,
    Class: Component<H, Props, State>,
{
    component: Class,
    props: Props,
    state: State,
    _phantom: PhantomData<H>,
}

impl<H, Class, Props, State> NodeCreator<H> for StatefulElement<H, Class, Props, State>
where
    H: HostElement,
    Class: Component<H, Props, State> + 'static,
    Props: 'static+Clone,
    State: 'static,
{
    fn create_node(&self) -> Box<dyn VirtualNode<H>> {
        let (component, initial_state) = Class::create(&self.props);

        Box::new(StatefulNode {
            component: component,
            props: self.props.clone(),
            state: initial_state,
            _phantom: PhantomData,
        })
    }
}

impl<H, Class, Props, State> VirtualNode<H> for StatefulNode<H, Class, Props, State>
where
    H: HostElement,
    Class: Component<H, Props, State>,
{
    fn mount(&mut self) {}

    fn update(&mut self) {}

    fn unmount(&mut self) {}
}
