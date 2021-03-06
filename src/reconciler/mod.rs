use component::Component;
use element::DomNode;
use element::{Element, HostElement};
use flat_tree::FlatTree;
use flat_tree::GetNodeChildren;
use flat_tree::NodeChildren;
use flat_tree::NodeKey;
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
}

struct UpdateQueue<H: HostElement> {
    queue: Arc<Mutex<Vec<Box<FnMut(&mut VirtualTree<H>)>>>>,
}

impl<H> Clone for UpdateQueue<H>
where
    H: HostElement,
{
    fn clone(&self) -> Self {
        UpdateQueue {
            queue: self.queue.clone(),
        }
    }
}

pub struct GenericStateUpdater<H: HostElement> {
    queue: UpdateQueue<H>,
    node: NodeKey<VirtualNode<H>>,
}

impl<H> Clone for GenericStateUpdater<H>
where
    H: HostElement,
{
    fn clone(&self) -> Self {
        GenericStateUpdater {
            queue: self.queue.clone(),
            node: self.node,
        }
    }
}

impl<H> GenericStateUpdater<H>
where
    H: HostElement,
{
    fn new(queue: &UpdateQueue<H>, key: NodeKey<VirtualNode<H>>) -> GenericStateUpdater<H> {
        GenericStateUpdater {
            queue: queue.clone(),
            node: key,
        }
    }

    pub fn specialize<Class>(&self) -> StateUpdater<H, Class>
    where
        Class: Component<H>,
    {
        StateUpdater {
            queue: self.queue.clone(),
            node: self.node,
            _phantom: PhantomData,
        }
    }
}

pub struct StateUpdater<H: HostElement, Class: Component<H>> {
    queue: UpdateQueue<H>,
    node: NodeKey<VirtualNode<H>>,
    _phantom: PhantomData<Class>,
}

impl<H, Class> StateUpdater<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn unspecialize(&self) -> GenericStateUpdater<H> {
        GenericStateUpdater {
            queue: self.queue.clone(),
            node: self.node,
        }
    }

    pub fn set_state<Func>(&self, func: Func)
    where
        Func: FnOnce(Class::State) -> Class::State + 'static,
    {
        let index = self.node;
        let mut func = Some(func);
        let updater = self.unspecialize();
        self.queue.push(move |tree| {
            let element = match tree.tree.get_mut(index) {
                VirtualNode::Host(_) => panic!(),
                VirtualNode::Fragment(_) => panic!(),
                VirtualNode::Stateful(node) => {
                    match node.as_any_mut().downcast_mut::<StatefulNode<H, Class>>() {
                        Some(ref mut node) => {
                            node.update_state(func.take().unwrap(), updater.clone())
                        }
                        None => panic!(),
                    }
                }
            };
            let child = tree.tree.get_children(index).first().map(|&x| x);
            if let Some(child) = child {
                tree.update_subtree(child, element);
            }
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

    pub fn push<Func: FnMut(&mut VirtualTree<H>) + 'static>(&self, func: Func) {
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
            VirtualNode::Fragment(ref children) => children,
        }
    }

    fn get_children_mut(&mut self) -> &mut NodeChildren<Self> {
        match *self {
            VirtualNode::Host(ref mut host_node) => &mut host_node.children,
            VirtualNode::Stateful(ref mut stateful_node) => stateful_node.get_children_mut(),
            VirtualNode::Fragment(ref mut children) => children,
        }
    }
}

impl<H> VirtualTree<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> Self {
        let queue = UpdateQueue::new();
        let tree = FlatTree::build(element, |node, index| {
            VirtualNode::mount(node, GenericStateUpdater::new(&queue, index))
        });

        VirtualTree {
            tree: tree,
            update_queue: queue,
        }
    }

    fn update_subtree(&mut self, node: NodeKey<VirtualNode<H>>, element: Element<H>) {
        let queue = &self.update_queue;
        self.tree.update_subtree(
            node,
            element,
            &mut |node, index| VirtualNode::mount(node, GenericStateUpdater::new(queue, index)),
            &mut |node, element, index| {
                VirtualNode::update(node, element, GenericStateUpdater::new(queue, index))
            },
            &mut |node, index| VirtualNode::unmount(node, GenericStateUpdater::new(queue, index)),
        );
    }

    pub fn flush(&mut self) {
        let items = {
            let mut guard = self.update_queue.queue.lock().unwrap();
            let items = guard
                .drain(..)
                .collect::<Vec<Box<FnMut(&mut VirtualTree<H>)>>>();
            items
        };
        for mut func in items.into_iter() {
            (func)(self);
        }
    }

    pub fn update(&mut self, element: Element<H>) {
        self.flush();

        let queue = &self.update_queue;
        self.tree.update_tree(
            element,
            &mut |node, index| VirtualNode::mount(node, GenericStateUpdater::new(&queue, index)),
            &mut |node, element, index| {
                VirtualNode::update(node, element, GenericStateUpdater::new(&queue, index))
            },
            &mut |node, index| VirtualNode::unmount(node, GenericStateUpdater::new(&queue, index)),
        );
    }

    pub fn unmount(self) {
        let queue = self.update_queue;
        self.tree.unbuild(|node, _, index| {
            VirtualNode::unmount(node, GenericStateUpdater::new(&queue, index))
        });
    }

    pub fn render<'a, Dom>(&'a self) -> Option<Dom>
    where
        Dom: DomNode<'a, Widget = H>,
    {
        let result = self
            .tree
            .recurse(|node, children| {
                node.render(children.into_iter().flatten().collect::<Vec<Dom>>())
            })
            .unwrap_or(vec![]);
        assert!(result.len() <= 1);
        result.into_iter().next()
    }
}
