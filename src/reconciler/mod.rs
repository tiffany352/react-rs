use element::{Element, HostElement};

pub trait NodeCreator<H: HostElement> {
    fn create_node(&self) -> Box<dyn VirtualNode<H>>;
}

pub trait VirtualNode<H: HostElement> {
    fn mount(&mut self);
    fn update(&mut self);
    fn unmount(&mut self);
}

mod host_node;
mod stateful_node;

fn mount_node<H>(element: Element<H>) -> Box<dyn VirtualNode<H>>
where
    H: HostElement,
{
    match element {
        Element::Host { element: _, children: _ } => unimplemented!(),
        Element::Stateful(node_creator) => node_creator.create_node(),
    }
}

pub struct VirtualTree<H: HostElement> {
    root: Box<dyn VirtualNode<H>>,
}

pub fn mount<H>(element: Element<H>) -> VirtualTree<H>
where
    H: HostElement,
{
    VirtualTree {
        root: mount_node(element),
    }
}
