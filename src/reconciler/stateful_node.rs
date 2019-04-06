use component::Component;
use element::Element;
use element::{HostElement, StatefulElement};
use reconciler::{StatefulElementWrapper, VirtualNode};
use std::any::Any;
use std::clone::Clone;
use std::marker::PhantomData;

struct StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    component: Class,
    props: Class::Props,
    state: Option<Class::State>,
    child: Option<usize>,
    _phantom: PhantomData<H>,
}

pub trait StatefulNodeWrapper<H: HostElement> {
    fn mount(&mut self, nodes: &mut Vec<VirtualNode<H>>);
    fn update(
        &mut self,
        element: Element<H>,
        old_nodes: &mut Vec<VirtualNode<H>>,
        new_nodes: &mut Vec<VirtualNode<H>>,
    ) -> Result<(), Element<H>>;
    fn unmount(&mut self, nodes: &mut Vec<VirtualNode<H>>);
    fn render(&self, nodes: &[VirtualNode<H>]) -> Option<H::DomNode>;
    fn as_any(&self) -> &dyn Any;
}

impl<H, Class> StatefulNodeWrapper<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn mount(&mut self, nodes: &mut Vec<VirtualNode<H>>) {
        let element = self
            .component
            .render(&self.props, self.state.as_ref().unwrap());

        self.child = Some(VirtualNode::mount(element, nodes));

        self.component.did_mount();
    }

    fn update(
        &mut self,
        element: Element<H>,
        old_nodes: &mut Vec<VirtualNode<H>>,
        new_nodes: &mut Vec<VirtualNode<H>>,
    ) -> Result<(), Element<H>> {
        match element {
            Element::Host { .. } => Err(element),
            Element::Stateful(element) => {
                match element.as_any().downcast_ref::<StatefulElement<H, Class>>() {
                    Some(element) => {
                        self.props = element.props.clone();

                        self.state = Some(Class::get_derived_state_from_props(
                            &self.props,
                            self.state.take().unwrap(),
                        ));

                        let element = self
                            .component
                            .render(&self.props, self.state.as_ref().unwrap());

                        if let Some(index) = self.child.take() {
                            assert!(index + 1 == old_nodes.len());
                            let child = old_nodes.pop().unwrap();
                            self.child =
                                Some(VirtualNode::update(child, element, old_nodes, new_nodes));
                        } else {
                            self.child = Some(VirtualNode::mount(element, new_nodes));
                        }

                        Ok(())
                    }
                    None => Err(()),
                }
                .map_err(|_| Element::Stateful(element))
            }
        }
    }

    fn unmount(&mut self, nodes: &mut Vec<VirtualNode<H>>) {
        self.component.will_unmount();

        if let Some(index) = self.child.take() {
            assert!(index + 1 == nodes.len());
            let child = nodes.pop().unwrap();
            VirtualNode::unmount(child, nodes);
        }
    }

    fn render(&self, nodes: &[VirtualNode<H>]) -> Option<H::DomNode> {
        nodes[self.child?].render(nodes)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<H, Class> StatefulElementWrapper<H> for StatefulElement<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn create_node(&self) -> Box<dyn StatefulNodeWrapper<H>> {
        let (component, initial_state) = Class::create(&self.props);

        Box::new(StatefulNode {
            component: component,
            props: self.props.clone(),
            state: Some(initial_state),
            child: None,
            _phantom: PhantomData,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn StatefulElementWrapper<H>> {
        let clone: StatefulElement<H, Class> = (*self).clone();
        Box::new(clone)
    }
}
