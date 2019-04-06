use component::Component;
use component::RenderContext;
use element::Element;
use element::{HostElement, StatefulElement};
use flat_tree::NodeChildren;
use reconciler::GenericStateUpdater;
use reconciler::{StatefulElementWrapper, VirtualNode};
use std::any::Any;
use std::clone::Clone;
use std::marker::PhantomData;

pub struct StatefulNode<H, Class>
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
    fn mount(&mut self, updater: GenericStateUpdater<H>) -> Element<H>;
    fn update(
        &mut self,
        element: Element<H>,
        updater: GenericStateUpdater<H>,
    ) -> Result<Element<H>, Element<H>>;
    fn unmount(&mut self, updater: GenericStateUpdater<H>);
    fn render(&self, children: Vec<H::DomNode>) -> Option<H::DomNode>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_children(&self) -> &NodeChildren<VirtualNode<H>>;
    fn get_children_mut(&mut self) -> &mut NodeChildren<VirtualNode<H>>;
}

impl<H, Class> StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    pub fn update_state<Func>(&mut self, func: Func)
    where
        Func: FnOnce(Class::State) -> Class::State,
    {
        self.state = Some((func)(self.state.take().unwrap()))
    }
}

impl<H, Class> StatefulNodeWrapper<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn mount(&mut self, updater: GenericStateUpdater<H>) -> Element<H> {
        let element = self.component.render(RenderContext {
            props: &self.props,
            state: self.state.as_ref().unwrap(),
            updater: updater.specialize(),
        });

        self.component.did_mount();

        element
    }

    fn update(
        &mut self,
        element: Element<H>,
        updater: GenericStateUpdater<H>,
    ) -> Result<Element<H>, Element<H>> {
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

                        let element = self.component.render(RenderContext {
                            props: &self.props,
                            state: self.state.as_ref().unwrap(),
                            updater: updater.specialize(),
                        });

                        Ok(element)
                    }
                    None => Err(()),
                }
                .map_err(|_| Element::Stateful(element))
            }
        }
    }

    fn unmount(&mut self, _updater: GenericStateUpdater<H>) {
        self.component.will_unmount();
    }

    fn render(&self, mut children: Vec<H::DomNode>) -> Option<H::DomNode> {
        assert!(children.len() <= 1);
        children.pop()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
}
