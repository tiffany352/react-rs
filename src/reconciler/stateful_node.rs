use component::Component;
use element::{HostElement, StatefulElement};
use reconciler::{VirtualNode, NodeCreator, mount_node};
use std::marker::PhantomData;

struct StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    component: Class,
    props: Class::Props,
    state: Class::State,
    child: Option<Box<dyn VirtualNode<H>>>,
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
            child: None,
            _phantom: PhantomData,
        })
    }
}

impl<H, Class> VirtualNode<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn mount(&mut self) {
        let element = self.component.render(&self.props, &self.state);

        let mut child = mount_node(element);

        child.mount();

        self.child = Some(child);

        self.component.did_mount();
    }

    fn update(&mut self) {}

    fn unmount(&mut self) {}

    fn render(&self) -> Option<H::DomNode> {
        self.child.as_ref()?.render()
    }
}
