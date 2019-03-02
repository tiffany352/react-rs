use element::{Element, HostElement};

pub trait NodeCreator<H: HostElement> {
    fn create_node(&self) -> Box<dyn VirtualNode<H>>;
}

pub trait VirtualNode<H: HostElement> {
    fn mount(&mut self);
    fn update(&mut self);
    fn unmount(&mut self);
    fn render(&self) -> Option<H::DomNode>;
}

mod host_node;
mod stateful_node;

fn mount_node<H>(element: Element<H>) -> Box<dyn VirtualNode<H>>
where
    H: HostElement,
{
    match element {
        Element::Host { element, children } => Box::new(host_node::HostNode {
            element: element,
            children: children.into_iter().map(|elt| mount_node(elt)).collect::<Vec<_>>(),
        }),
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
        let mut root = mount_node(element);

        root.mount();

        VirtualTree {
            root: root,
        }
    }

    pub fn update(self, _element: Element<H>) -> Self {
        unimplemented!()
    }

    pub fn unmount(self) {
        unimplemented!()
    }

    pub fn render(&self) -> Option<H::DomNode> {
        self.root.render()
    }
}
