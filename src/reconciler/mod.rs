use element::{Element, HostElement};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

mod host_node;
mod stateful_node;
mod virtual_node;

pub use self::host_node::HostNode;
pub use self::stateful_node::{StateUpdater, StatefulNodeWrapper};
pub use self::virtual_node::VirtualNode;

pub trait StatefulElementWrapper<H: HostElement>: Any {
    fn create_node(&self) -> Rc<RefCell<dyn StatefulNodeWrapper<H>>>;

    fn as_any(&self) -> &dyn Any;

    fn box_clone(&self) -> Box<dyn StatefulElementWrapper<H>>;
}

impl<H: HostElement> Clone for Box<StatefulElementWrapper<H>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct VirtualTree<H: HostElement> {
    root: VirtualNode<H>,
}

impl<H> VirtualTree<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> Self {
        let root = VirtualNode::mount(element);

        VirtualTree { root: root }
    }

    pub fn update(self, element: Element<H>) -> Self {
        VirtualTree {
            root: VirtualNode::update(self.root, element),
        }
    }

    pub fn unmount(self) {
        VirtualNode::unmount(self.root);
    }

    pub fn render(&self) -> Option<H::DomNode> {
        self.root.render()
    }
}
