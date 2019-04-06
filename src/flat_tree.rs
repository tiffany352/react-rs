use std::iter::repeat_with;
use std::marker::PhantomData;

pub struct NodeChildren<Item> {
    children: Vec<usize>,
    _phantom: PhantomData<Item>,
}

impl<Item> NodeChildren<Item> {
    pub fn new() -> NodeChildren<Item> {
        NodeChildren {
            children: vec![],
            _phantom: PhantomData,
        }
    }
}

pub trait GetNodeChildren: Sized {
    fn get_children(&self) -> &NodeChildren<Self>;
    fn get_children_mut(&mut self) -> &mut NodeChildren<Self>;
}

pub struct FlatTree<Item> {
    items: Vec<Item>,
}

impl<Item> FlatTree<Item>
where
    Item: GetNodeChildren,
{
    pub fn new() -> FlatTree<Item> {
        FlatTree { items: vec![] }
    }

    fn push(&mut self, item: Item) -> usize {
        self.items.push(item);
        self.items.len() - 1
    }

    fn build_inner<Value, Func>(&mut self, root: Value, create_node: &mut Func) -> usize
    where
        Func: FnMut(Value) -> (Item, Vec<Value>),
    {
        let (mut item, children) = create_node(root);

        let children = children
            .into_iter()
            .map(|child| self.build_inner(child, create_node))
            .collect::<Vec<_>>();

        item.get_children_mut().children = children;

        self.push(item)
    }

    pub fn build<Value, Func>(root: Value, mut create_node: Func) -> FlatTree<Item>
    where
        Func: FnMut(Value) -> (Item, Vec<Value>),
    {
        let mut tree = FlatTree::new();

        tree.build_inner(root, &mut create_node);

        tree
    }

    fn unbuild_inner<Func, Res>(&mut self, item: Item, take_item: &mut Func) -> Res
    where
        Func: FnMut(Item, Vec<Res>) -> Res,
    {
        let children = item
            .get_children()
            .children
            .iter()
            .map(|index| {
                assert!(index + 1 == self.items.len());
                let child = self.items.pop().unwrap();
                self.unbuild_inner(child, take_item)
            })
            .collect::<Vec<_>>();

        take_item(item, children)
    }

    pub fn unbuild<Func, Res>(mut self, mut take_item: Func) -> Res
    where
        Func: FnMut(Item, Vec<Res>) -> Res,
    {
        let root = self.items.pop().unwrap();
        self.unbuild_inner(root, &mut take_item)
    }

    fn recurse_inner<Func, Res>(&self, item: &Item, map_item: &mut Func) -> Res
    where
        Func: FnMut(&Item, Vec<Res>) -> Res,
    {
        let children = item
            .get_children()
            .children
            .iter()
            .map(|index| {
                let child = &self.items[*index];
                self.recurse_inner(child, map_item)
            })
            .collect::<Vec<_>>();

        map_item(item, children)
    }

    pub fn recurse<Func, Res>(&self, mut map_item: Func) -> Res
    where
        Func: FnMut(&Item, Vec<Res>) -> Res,
    {
        self.recurse_inner(self.items.last().unwrap(), &mut map_item)
    }

    pub fn transform_inner<Value, MountItem, UpdateItem, UnmountItem>(
        old_tree: &mut FlatTree<Item>,
        new_tree: &mut FlatTree<Item>,
        mut item: Option<Item>,
        value: Value,
        mount_item: &mut MountItem,
        update_item: &mut UpdateItem,
        unmount_item: &mut UnmountItem,
    ) -> usize
    where
        MountItem: FnMut(Value) -> (Item, Vec<Value>),
        UpdateItem: FnMut(Item, Value) -> (Item, Vec<Value>),
        UnmountItem: FnMut(Item),
    {
        let previous_children = if let Some(item) = item.as_mut() {
            item.get_children_mut().children.drain(..).collect()
        } else {
            vec![]
        };

        let (mut item, child_values) = if let Some(item) = item {
            update_item(item, value)
        } else {
            mount_item(value)
        };

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
            .take_while(|(child_index, child_value)| child_index.is_some() && child_value.is_some())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .filter_map(|(child_index, child_value)| {
                let child_item = if let Some(index) = child_index {
                    assert!(index + 1 == old_tree.items.len());
                    old_tree.items.pop()
                } else {
                    None
                };
                match (child_item, child_value) {
                    (child_item, Some(child_value)) => Some(FlatTree::transform_inner(
                        old_tree,
                        new_tree,
                        child_item,
                        child_value,
                        mount_item,
                        update_item,
                        unmount_item,
                    )),
                    (Some(child_item), None) => {
                        old_tree.unbuild_inner(child_item, &mut |item, _| unmount_item(item));
                        None
                    }
                    (None, None) => None,
                }
            })
            .collect::<Vec<usize>>();
        child_indices.reverse();

        item.get_children_mut().children = child_indices;

        new_tree.push(item)
    }

    pub fn transform<Value, MountItem, UpdateItem, UnmountItem>(
        mut self,
        value: Value,
        mut mount_item: MountItem,
        mut update_item: UpdateItem,
        mut unmount_item: UnmountItem,
    ) -> FlatTree<Item>
    where
        MountItem: FnMut(Value) -> (Item, Vec<Value>),
        UpdateItem: FnMut(Item, Value) -> (Item, Vec<Value>),
        UnmountItem: FnMut(Item),
    {
        let mut new_tree = FlatTree::new();
        let root = self.items.pop();

        FlatTree::transform_inner(
            &mut self,
            &mut new_tree,
            root,
            value,
            &mut mount_item,
            &mut update_item,
            &mut unmount_item,
        );

        new_tree
    }

    pub fn get(&self, index: usize) -> &Item {
        &self.items[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Item {
        &mut self.items[index]
    }
}
