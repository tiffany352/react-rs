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
    child: Option<VirtualNode<H>>,
    _phantom: PhantomData<H>,
}

pub trait StatefulNodeWrapper<H: HostElement> {
    fn mount(&mut self);
    fn update(&mut self, element: Element<H>) -> Result<(), Element<H>>;
    fn unmount(&mut self);
    fn render(&self) -> Option<H::DomNode>;
}

impl<H, Class> StatefulNodeWrapper<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn mount(&mut self) {
        let element = self
            .component
            .render(&self.props, self.state.as_ref().unwrap());

        self.child = Some(VirtualNode::mount(element));

        self.component.did_mount();
    }

    fn update(&mut self, element: Element<H>) -> Result<(), Element<H>> {
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

                        if let Some(child) = self.child.take() {
                            self.child = Some(VirtualNode::update(child, element));
                        } else {
                            self.child = Some(VirtualNode::mount(element));
                        }

                        Ok(())
                    }
                    None => Err(()),
                }
                .map_err(|_| Element::Stateful(element))
            }
        }
    }

    fn unmount(&mut self) {
        self.component.will_unmount();

        if let Some(child) = self.child.take() {
            VirtualNode::unmount(child);
        }
    }

    fn render(&self) -> Option<H::DomNode> {
        self.child.as_ref()?.render()
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
