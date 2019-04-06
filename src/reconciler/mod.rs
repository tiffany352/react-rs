use element::{Element, HostElement};
use std::any::Any;
use std::sync::{Arc, Mutex};

mod host_node;
mod stateful_node;
mod virtual_node;

pub use self::host_node::HostNode;
pub use self::stateful_node::StatefulNodeWrapper;
pub use self::virtual_node::VirtualNode;

pub trait StatefulElementWrapper<H: HostElement>: Any {
    fn create_node(&self) -> Box<dyn StatefulNodeWrapper<H>>;

    fn as_any(&self) -> &dyn Any;

    fn box_clone(&self) -> Box<dyn StatefulElementWrapper<H>>;
}

impl<H: HostElement> Clone for Box<StatefulElementWrapper<H>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

struct UpdateRequest<H: HostElement> {
    node_index: usize,
    func: Box<FnOnce(&mut VirtualNode<H>)>,
}

#[derive(Clone)]
struct UpdateQueue<H: HostElement> {
    queue: Arc<Mutex<Vec<UpdateRequest<H>>>>,
}

impl<H> UpdateQueue<H>
where
    H: HostElement,
{
    fn new() -> UpdateQueue<H> {
        UpdateQueue {
            queue: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn push<Func: FnOnce(&mut VirtualNode<H>) + 'static>(&self, node_index: usize, func: Func) {
        self.queue.lock().unwrap().push(UpdateRequest {
            node_index,
            func: Box::new(func),
        });
    }
}

pub struct VirtualTree<H: HostElement> {
    nodes: Vec<VirtualNode<H>>,
    update_queue: UpdateQueue<H>,
}

impl<H> VirtualTree<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> Self {
        let mut nodes = vec![];
        VirtualNode::mount(element, &mut nodes);

        VirtualTree {
            nodes: nodes,
            update_queue: UpdateQueue::new(),
        }
    }

    pub fn update(mut self, element: Element<H>) -> Self {
        let root = self.nodes.pop().unwrap();
        let mut new_nodes = vec![];
        VirtualNode::update(root, element, &mut self.nodes, &mut new_nodes);
        VirtualTree {
            nodes: new_nodes,
            update_queue: UpdateQueue::new(),
        }
    }

    pub fn unmount(mut self) {
        let root = self.nodes.pop().unwrap();
        VirtualNode::unmount(root, &mut self.nodes);
    }

    pub fn render(&self) -> Option<H::DomNode> {
        self.nodes.last().unwrap().render(&self.nodes)
    }
}
