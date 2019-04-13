use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::repeat_with;
use std::marker::PhantomData;

pub struct NodeKey<Item> {
    index: usize,
    _phantom: PhantomData<Item>,
}

impl<Item> Clone for NodeKey<Item> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Item> Copy for NodeKey<Item> {}

impl<Item> NodeKey<Item> {
    fn new(index: usize) -> NodeKey<Item> {
        NodeKey {
            index,
            _phantom: PhantomData,
        }
    }
}

impl<Item> PartialEq for NodeKey<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Item> Eq for NodeKey<Item> {}

impl<Item> Hash for NodeKey<Item> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.index.hash(state)
    }
}

pub struct NodeChildren<Item> {
    children: Vec<NodeKey<Item>>,
}

impl<Item> NodeChildren<Item> {
    pub fn new() -> NodeChildren<Item> {
        NodeChildren { children: vec![] }
    }
}

pub trait GetNodeChildren: Sized {
    fn get_children(&self) -> &NodeChildren<Self>;
    fn get_children_mut(&mut self) -> &mut NodeChildren<Self>;
}

pub struct FlatTree<Item> {
    items: HashMap<NodeKey<Item>, Item>,
    next_key: usize,
    root: Option<NodeKey<Item>>,
}

impl<Item> FlatTree<Item>
where
    Item: GetNodeChildren,
{
    pub fn new() -> FlatTree<Item> {
        FlatTree {
            items: HashMap::new(),
            next_key: 1,
            root: None,
        }
    }

    fn reserve(&mut self) -> NodeKey<Item> {
        let key = self.next_key;
        self.next_key += 1;
        NodeKey::new(key)
    }

    fn insert(&mut self, key: NodeKey<Item>, item: Item) {
        self.items.insert(key, item);
    }

    fn build_inner<Value, Func>(&mut self, root: Value, create_node: &mut Func) -> NodeKey<Item>
    where
        Func: FnMut(Value, NodeKey<Item>) -> (Item, Vec<Value>),
    {
        let key = self.reserve();

        let (mut item, children) = create_node(root, key);

        let children = children
            .into_iter()
            .map(|child| self.build_inner(child, create_node))
            .collect::<Vec<_>>();

        item.get_children_mut().children = children;

        self.insert(key, item);

        key
    }

    pub fn build<Value, Func>(root: Value, mut create_node: Func) -> FlatTree<Item>
    where
        Func: FnMut(Value, NodeKey<Item>) -> (Item, Vec<Value>),
    {
        let mut tree = FlatTree::new();

        tree.root = Some(tree.build_inner(root, &mut create_node));

        tree
    }

    fn unbuild_inner<Func, Res>(&mut self, key: NodeKey<Item>, take_item: &mut Func) -> Res
    where
        Func: FnMut(Item, Vec<Res>, NodeKey<Item>) -> Res,
    {
        let item = self.items.remove(&key).unwrap();

        let children = item
            .get_children()
            .children
            .iter()
            .map(|index| self.unbuild_inner(*index, take_item))
            .collect::<Vec<_>>();

        take_item(item, children, key)
    }

    pub fn unbuild<Func, Res>(mut self, mut take_item: Func) -> Option<Res>
    where
        Func: FnMut(Item, Vec<Res>, NodeKey<Item>) -> Res,
    {
        if let Some(root) = self.root {
            Some(self.unbuild_inner(root, &mut take_item))
        } else {
            None
        }
    }

    fn recurse_inner<'a, Func, Res>(&'a self, item: &'a Item, map_item: &mut Func) -> Res
    where
        Func: FnMut(&'a Item, Vec<Res>) -> Res,
    {
        let children = item
            .get_children()
            .children
            .iter()
            .map(|index| {
                let child = &self.items[index];
                self.recurse_inner(child, map_item)
            })
            .collect::<Vec<_>>();

        map_item(item, children)
    }

    pub fn recurse<'a, Func, Res>(&'a self, mut map_item: Func) -> Option<Res>
    where
        Func: FnMut(&'a Item, Vec<Res>) -> Res,
    {
        if let Some(ref root) = self.root {
            let root = self.items.get(root).unwrap();
            Some(self.recurse_inner(root, &mut map_item))
        } else {
            None
        }
    }

    fn recurse_inner_mut<Func, Res>(&mut self, index: NodeKey<Item>, map_item: &mut Func) -> Res
    where
        Func: FnMut(&mut Item, Vec<Res>, NodeKey<Item>) -> Res,
    {
        let children = self
            .items
            .get(&index)
            .unwrap()
            .get_children()
            .children
            .clone()
            .iter()
            .map(|index| self.recurse_inner_mut(*index, map_item))
            .collect::<Vec<_>>();

        map_item(self.items.get_mut(&index).unwrap(), children, index)
    }

    pub fn recurse_mut<Func, Res>(&mut self, mut map_item: Func) -> Option<Res>
    where
        Func: FnMut(&mut Item, Vec<Res>, NodeKey<Item>) -> Res,
    {
        if let Some(root) = self.root {
            Some(self.recurse_inner_mut(root, &mut map_item))
        } else {
            None
        }
    }

    pub fn update_subtree<Value, MountItem, UpdateItem, UnmountItem>(
        &mut self,
        item_key: NodeKey<Item>,
        value: Value,
        mount_item: &mut MountItem,
        update_item: &mut UpdateItem,
        unmount_item: &mut UnmountItem,
    ) -> NodeKey<Item>
    where
        MountItem: FnMut(Value, NodeKey<Item>) -> (Item, Vec<Value>),
        UpdateItem: FnMut(Item, Value, NodeKey<Item>) -> (Item, Option<Vec<Value>>),
        UnmountItem: FnMut(Item, NodeKey<Item>),
    {
        let mut item = self.items.remove(&item_key).unwrap();
        let previous_children = item
            .get_children_mut()
            .children
            .drain(..)
            .collect::<Vec<NodeKey<Item>>>();

        let (mut item, child_values) = update_item(item, value, item_key);

        if let Some(child_values) = child_values {
            let mut child_indices = previous_children
                .into_iter()
                .map(Some)
                .chain(repeat_with(|| None))
                .zip(
                    child_values
                        .into_iter()
                        .map(Some)
                        .chain(repeat_with(|| None)),
                )
                .take_while(|(child_index, child_value)| {
                    child_index.is_some() && child_value.is_some()
                })
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(
                    |(child_index, child_value)| match (child_index, child_value) {
                        // Update
                        (Some(child_index), Some(child_value)) => Some(FlatTree::update_subtree(
                            self,
                            child_index,
                            child_value,
                            mount_item,
                            update_item,
                            unmount_item,
                        )),
                        // Mount
                        (None, Some(child_value)) => {
                            Some(self.build_inner(child_value, mount_item))
                        }
                        // Unmount
                        (Some(child_index), None) => {
                            self.unbuild_inner(child_index, &mut |item, _, key| {
                                unmount_item(item, key)
                            });
                            None
                        }
                        // Unreachable
                        (None, None) => None,
                    },
                )
                .collect::<Vec<NodeKey<Item>>>();

            item.get_children_mut().children = child_indices;
        } else {
            item.get_children_mut().children = previous_children;
        }

        self.insert(item_key, item);
        item_key
    }

    pub fn update_tree<Value, MountItem, UpdateItem, UnmountItem>(
        &mut self,
        value: Value,
        mount_item: &mut MountItem,
        update_item: &mut UpdateItem,
        unmount_item: &mut UnmountItem,
    ) where
        MountItem: FnMut(Value, NodeKey<Item>) -> (Item, Vec<Value>),
        UpdateItem: FnMut(Item, Value, NodeKey<Item>) -> (Item, Option<Vec<Value>>),
        UnmountItem: FnMut(Item, NodeKey<Item>),
    {
        if let Some(root) = self.root {
            self.update_subtree(root, value, mount_item, update_item, unmount_item);
        } else {
            self.build_inner(value, mount_item);
        }
    }

    pub fn get_children(&self, index: NodeKey<Item>) -> &[NodeKey<Item>] {
        &self.items.get(&index).unwrap().get_children().children[..]
    }

    pub fn get(&self, index: NodeKey<Item>) -> &Item {
        self.items.get(&index).unwrap()
    }

    pub fn get_mut(&mut self, index: NodeKey<Item>) -> &mut Item {
        self.items.get_mut(&index).unwrap()
    }
}
