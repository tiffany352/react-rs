use component::Component;
use element::{HostElement, StatefulElement};
use reconciler::NodeCreator;
use reconciler::VirtualNode;
use std::marker::PhantomData;

struct StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    component: Class,
    props: Class::Props,
    state: Class::State,
    _phantom: PhantomData<H>,
}

impl<H, Class> NodeCreator<H> for StatefulElement<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
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

impl<H, Class> VirtualNode<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn mount(&mut self) {}

    fn update(&mut self) {}

    fn unmount(&mut self) {}

    fn render(&self) -> H::DomNode {
        unimplemented!()
    }
}
