use component::Component;
use element::{Element, HostElement};
use flat_tree::FlatTree;
use flat_tree::GetNodeChildren;
use flat_tree::NodeChildren;
use reconciler::stateful_node::StatefulNode;
use std::any::Any;
use std::marker::PhantomData;
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

#[derive(Clone)]
struct UpdateQueue<H: HostElement> {
    queue: Arc<Mutex<Vec<Box<FnOnce(&mut VirtualTree<H>)>>>>,
}

pub struct StateUpdater<H: HostElement, Class: Component<H>> {
    queue: UpdateQueue<H>,
    node: usize,
    _phantom: PhantomData<Class>,
}

impl<H, Class> StateUpdater<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    pub fn set_state<Func>(&self, func: Func)
    where
        Func: FnOnce(Class::State) -> Class::State + 'static,
    {
        let index = self.node;
        self.queue.push(self.node, move |tree| {
            let node = tree.tree.get_mut(index);
            match node {
                VirtualNode::Host(_) => panic!(),
                VirtualNode::Stateful(node) => {
                    match node.as_any_mut().downcast_mut::<StatefulNode<H, Class>>() {
                        Some(ref mut node) => node.update_state(func),
                        None => panic!(),
                    }
                }
            };
            unimplemented!()
        })
    }
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

    pub fn push<Func: FnOnce(&mut VirtualTree<H>) + 'static>(&self, node_index: usize, func: Func) {
        self.queue.lock().unwrap().push(Box::new(func));
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
