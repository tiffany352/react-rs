use element::{Element, HostElement};

pub trait NodeCreator<H: HostElement> {
    fn create_node(&self) -> Box<dyn VirtualNode<H>>;
}

pub trait VirtualNode<H: HostElement> {
    fn mount(&mut self);
    fn update(&mut self);
    fn unmount(&mut self);
    fn render(&self) -> H::DomNode;
}

mod host_node;
mod stateful_node;

fn mount_node<H>(element: Element<H>) -> Box<dyn VirtualNode<H>>
where
    H: HostElement,
{
    match element {
        Element::Host {
            element: _,
            children: _,
        } => unimplemented!(),
        Element::Stateful(node_creator) => node_creator.create_node(),
    }
}

pub struct VirtualTree<H: HostElement> {
    root: Box<dyn VirtualNode<H>>,
}

impl<H> VirtualTree<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> Self {
        VirtualTree {
            root: mount_node(element),
        }
    }

    pub fn update(self, _element: Element<H>) -> Self {
        unimplemented!()
    }

    pub fn unmount(self) {
        unimplemented!()
    }

    pub fn render(&self) -> H::DomNode {
        unimplemented!()
    }
}
