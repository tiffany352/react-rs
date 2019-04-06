use element::{Element, HostElement};
use flat_tree::FlatTree;
use flat_tree::GetNodeChildren;
use flat_tree::NodeChildren;
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
    tree: FlatTree<VirtualNode<H>>,
    update_queue: UpdateQueue<H>,
}

impl<H> GetNodeChildren for VirtualNode<H>
where
    H: HostElement,
{
    fn get_children(&self) -> &NodeChildren<Self> {
        match *self {
            VirtualNode::Host(ref host_node) => &host_node.children,
            VirtualNode::Stateful(ref stateful_node) => stateful_node.get_children(),
        }
    }

    fn get_children_mut(&mut self) -> &mut NodeChildren<Self> {
        match *self {
            VirtualNode::Host(ref mut host_node) => &mut host_node.children,
            VirtualNode::Stateful(ref mut stateful_node) => stateful_node.get_children_mut(),
        }
    }
}

impl<H> VirtualTree<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> Self {
        let tree = FlatTree::build(element, VirtualNode::mount);

        VirtualTree {
            tree: tree,
            update_queue: UpdateQueue::new(),
        }
    }

    pub fn update(self, element: Element<H>) -> Self {
        VirtualTree {
            tree: self.tree.transform(
                element,
                VirtualNode::mount,
                VirtualNode::update,
                VirtualNode::unmount,
            ),
            update_queue: UpdateQueue::new(),
        }
    }

    pub fn unmount(self) {
        self.tree.unbuild(|node, _| VirtualNode::unmount(node));
    }

    pub fn render(&self) -> Option<H::DomNode> {
        self.tree.recurse(|node, children| {
            node.render(children.into_iter().filter_map(|x| x).collect::<Vec<_>>())
        })
    }
}
