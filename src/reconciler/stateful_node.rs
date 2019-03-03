use component::Component;
use element::Element;
use element::{HostElement, StatefulElement};
use reconciler::{StatefulElementWrapper, VirtualNode};
use std::any::Any;
use std::cell::RefCell;
use std::clone::Clone;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

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

pub struct StateUpdater<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    ptr: Weak<RefCell<StatefulNode<H, Class>>>,
}

impl<H, Class> Clone for StateUpdater<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn clone(&self) -> StateUpdater<H, Class> {
        StateUpdater {
            ptr: self.ptr.clone(),
        }
    }
}

impl<H, Class> StateUpdater<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    fn new(ptr: &Rc<RefCell<StatefulNode<H, Class>>>) -> StateUpdater<H, Class> {
        StateUpdater {
            ptr: Rc::downgrade(ptr),
        }
    }

    pub fn set_state(&self, new_state: Class::State) {
        self.update_state(|_| new_state)
    }

    pub fn update_state<Func>(&self, func: Func)
    where
        Func: FnOnce(Class::State) -> Class::State,
    {
        if let Some(ptr) = self.ptr.upgrade() {
            let mut node = ptr.borrow_mut();

            node.state = Some((func)(node.state.take().unwrap()));

            let updater: StateUpdater<H, Class> = (*self).clone();

            let element = node
                .component
                .render(&node.props, node.state.as_ref().unwrap(), updater);

            if let Some(child) = node.child.take() {
                node.child = Some(VirtualNode::update(child, element));
            } else {
                node.child = Some(VirtualNode::mount(element));
            }
        }
    }
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
    fn create_node(&self) -> Rc<RefCell<dyn StatefulNodeWrapper<H>>> {
        let (component, initial_state) = Class::create(&self.props);

        Rc::new(RefCell::new(StatefulNode {
            component: component,
            props: self.props.clone(),
            state: Some(initial_state),
            child: None,
            _phantom: PhantomData,
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn StatefulElementWrapper<H>> {
        let clone: StatefulElement<H, Class> = (*self).clone();
        Box::new(clone)
    }
}
