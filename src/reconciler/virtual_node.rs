use element::{Element, HostElement};
use flat_tree::GetNodeChildren;
use flat_tree::NodeChildren;
use reconciler::GenericStateUpdater;
use std::any::Any;

pub trait VirtualNode<H>
where
    H: HostElement,
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_children(&self) -> &NodeChildren<VirtualNodeBox<H>>;
    fn get_children_mut(&mut self) -> &mut NodeChildren<VirtualNodeBox<H>>;
    fn render<'a>(&'a self, updater: GenericStateUpdater<H>) -> Element<'a, H>;
}

pub type VirtualNodeBox<H> = Box<dyn VirtualNode<H>>;

impl<H> GetNodeChildren for VirtualNodeBox<H>
where
    H: HostElement,
{
    fn get_children(&self) -> &NodeChildren<Self> {
        self.get_children()
    }

    fn get_children_mut(&mut self) -> &mut NodeChildren<Self> {
        self.get_children_mut()
    }
}
