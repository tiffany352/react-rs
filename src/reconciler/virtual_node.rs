use super::HostNode;
use super::StatefulNodeWrapper;
use element::{Element, HostElement};
use std::cell::RefCell;
use std::rc::Rc;

pub enum VirtualNode<H: HostElement> {
    Host(HostNode<H>),
    Stateful(Rc<RefCell<dyn StatefulNodeWrapper<H>>>),
}

impl<H> VirtualNode<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>) -> VirtualNode<H> {
        match element {
            Element::Host { element, children } => {
                VirtualNode::Host(HostNode::mount(element, children))
            }
            Element::Stateful(node_creator) => {
                let mut node = node_creator.create_node();
                node.borrow_mut().mount();
                VirtualNode::Stateful(node)
            }
        }
    }

    pub fn update(node: VirtualNode<H>, element: Element<H>) -> VirtualNode<H> {
        match node {
            VirtualNode::Host(_host_node) => {
                // Just throw out the old value.
                VirtualNode::mount(element)
            }
            VirtualNode::Stateful(node) => {
                let result = node.borrow_mut().update(element);
                match result {
                    Ok(()) => VirtualNode::Stateful(node),
                    Err(element) => {
                        // We can't update using this element, have to tear
                        // down and mount a new node.
                        node.borrow_mut().unmount();

                        VirtualNode::mount(element)
                    }
                }
            }
        }
    }

    pub fn unmount(node: VirtualNode<H>) {
        match node {
            VirtualNode::Host(_) => (),
            VirtualNode::Stateful(node) => node.borrow_mut().unmount(),
        }
    }

    pub fn render(&self) -> Option<H::DomNode> {
        match *self {
            VirtualNode::Host(ref node) => node.render(),
            VirtualNode::Stateful(ref node) => node.borrow().render(),
        }
    }
}
