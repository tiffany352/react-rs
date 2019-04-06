use component::Component;
use element::Element;
use element::{HostElement, StatefulElement};
use flat_tree::NodeChildren;
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
    children: NodeChildren<VirtualNode<H>>,
    _phantom: PhantomData<H>,
}

pub trait StatefulNodeWrapper<H: HostElement> {
    fn mount(&mut self) -> Element<H>;
    fn update(&mut self, element: Element<H>) -> Result<Element<H>, Element<H>>;
    fn unmount(&mut self);
    fn render(&self, children: Vec<H::DomNode>) -> Option<H::DomNode>;
    fn as_any(&self) -> &dyn Any;
    fn get_children(&self) -> &NodeChildren<VirtualNode<H>>;
    fn get_children_mut(&mut self) -> &mut NodeChildren<VirtualNode<H>>;
}

impl<H, Class> StatefulNodeWrapper<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn mount(&mut self) -> Element<H> {
        let element = self
            .component
            .render(&self.props, self.state.as_ref().unwrap());

        self.component.did_mount();

        element
    }

    fn update(&mut self, element: Element<H>) -> Result<Element<H>, Element<H>> {
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

                        Ok(element)
                    }
                    None => Err(()),
                }
                .map_err(|_| Element::Stateful(element))
            }
        }
    }

    fn unmount(&mut self) {
        self.component.will_unmount();
    }

    fn render(&self, mut children: Vec<H::DomNode>) -> Option<H::DomNode> {
        assert!(children.len() <= 1);
        children.pop()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_children(&self) -> &NodeChildren<VirtualNode<H>> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut NodeChildren<VirtualNode<H>> {
        &mut self.children
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
            children: NodeChildren::new(),
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
