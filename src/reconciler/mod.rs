use component::Component;
use element::{Element, HostElement};
use flat_tree::FlatTree;
use flat_tree::GetNodeChildren;
use flat_tree::NodeKey;
use reconciler::stateful_node::StatefulNode;
use reconciler::virtual_node::VirtualNodeBox;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

mod stateful_node;
mod virtual_node;

pub use self::virtual_node::VirtualNode;

struct UpdateQueue<H>
where
    H: HostElement,
{
    queue: Arc<Mutex<Vec<Box<FnMut(&mut VirtualTreeImpl<H>)>>>>,
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
    node: NodeKey<VirtualNodeBox<H>>,
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
    fn new(queue: &UpdateQueue<H>, key: NodeKey<VirtualNodeBox<H>>) -> GenericStateUpdater<H> {
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
    node: NodeKey<VirtualNodeBox<H>>,
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
            let element = {
                let node = tree.tree.get_mut(index);
                match node.as_any_mut().downcast_mut::<StatefulNode<H, Class>>() {
                    Some(ref mut node) => node.update_state(func.take().unwrap(), updater.clone()),
                    None => panic!(),
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

    pub fn push<Func: FnMut(&mut VirtualTreeImpl<H>) + 'static>(&self, func: Func) {
        self.queue.lock().unwrap().push(Box::new(func));
    }
}

struct VirtualTreeImpl<H>
where
    H: HostElement,
{
    tree: FlatTree<VirtualNodeBox<H>>,
    update_queue: UpdateQueue<H>,
}

impl<H> VirtualTreeImpl<H> where H: HostElement {}

pub struct VirtualTree<H, Root>
where
    H: HostElement,
    Root: Component<H>,
{
    tree: VirtualTreeImpl<H>,
    _phantom: PhantomData<Root>,
}

impl<H, Root> VirtualTree<H, Root>
where
    H: HostElement,
    Root: Component<H>,
{
    pub fn mount(initial_props: Root::Props) -> Self {
        let queue = UpdateQueue::new();
        let tree = FlatTree::build(initial_props, |props, key| {
            let root = StatefulNode::mount(initial_props);

            (Box::new(root) as VirtualNodeBox<H>, vec![])
        });

        VirtualTree {
            tree: VirtualTreeImpl {
                tree: tree,
                update_queue: queue,
            },
            _phantom: PhantomData,
        }
    }

    pub fn flush(&mut self) {
        let items = {
            let mut guard = self.tree.update_queue.queue.lock().unwrap();
            let items = guard
                .drain(..)
                .collect::<Vec<Box<FnMut(&mut VirtualTreeImpl<H>)>>>();
            items
        };
        for mut func in items.into_iter() {
            (func)(&mut self.tree);
        }
    }

    pub fn update(&mut self, props: Root::Props) {
        self.flush();

        let queue = &self.tree.update_queue;
        self.tree.tree.update_tree(
            props,
            &mut |node, index| VirtualNode::mount(node, GenericStateUpdater::new(&queue, index)),
            &mut |node, element, index| {
                VirtualNode::update(node, element, GenericStateUpdater::new(&queue, index))
            },
            &mut |node, index| VirtualNode::unmount(node, GenericStateUpdater::new(&queue, index)),
        );
    }

    pub fn unmount(self) {
        let queue = self.tree.update_queue;
        self.tree.tree.unbuild(|node, _, index| {
            VirtualNode::unmount(node, GenericStateUpdater::new(&queue, index))
        });
    }

    pub fn render<'a>(&'a self) -> Element<'a, H> {
        let result = self
            .tree
            .recurse(|node, children| {
                node.render(
                    children
                        .into_iter()
                        .flatten()
                        .collect::<Vec<Element<'a, H>>>(),
                )
            })
            .unwrap_or(vec![]);
        assert!(result.len() <= 1);
        result.into_iter().next()
    }
}
