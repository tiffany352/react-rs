use super::HostNode;
use super::StatefulNodeWrapper;
use element::{Element, HostElement};
use reconciler::GenericStateUpdater;

pub enum VirtualNode<H: HostElement> {
    Host(HostNode<H>),
    Stateful(Box<dyn StatefulNodeWrapper<H>>),
}

impl<H> VirtualNode<H>
where
    H: HostElement,
{
    pub fn mount(
        element: Element<H>,
        updater: GenericStateUpdater<H>,
    ) -> (VirtualNode<H>, Vec<Element<H>>) {
        match element {
            Element::Host { element, children } => {
                (VirtualNode::Host(HostNode::mount(element)), children)
            }
            Element::Stateful(node_creator) => {
                let mut node = node_creator.create_node();
                let children = node.mount(updater);
                (VirtualNode::Stateful(node), vec![children])
            }
        }
    }

    pub fn update(
        node: VirtualNode<H>,
        element: Element<H>,
        updater: GenericStateUpdater<H>,
    ) -> (VirtualNode<H>, Vec<Element<H>>) {
        match (node, element) {
            (
                VirtualNode::Host(HostNode {
                    element: _,
                    children,
                }),
                Element::Host {
                    element,
                    children: element_children,
                },
            ) => (
                VirtualNode::Host(HostNode { element, children }),
                element_children,
            ),
            (VirtualNode::Stateful(mut node), element) => {
                match node.update(element, updater.clone()) {
                    Ok(element) => (VirtualNode::Stateful(node), vec![element]),
                    Err(element) => {
                        node.unmount(updater.clone());
                        VirtualNode::mount(element, updater)
                    }
                }
            }
            (old_node, new_element) => {
                // If they're not compatible, we have to unmount and
                // remount.
                VirtualNode::unmount(old_node, updater.clone());
                VirtualNode::mount(new_element, updater)
            }
        }
    }

    pub fn unmount(node: VirtualNode<H>, updater: GenericStateUpdater<H>) {
        match node {
            VirtualNode::Host(_) => (),
            VirtualNode::Stateful(mut node) => node.unmount(updater),
        }
    }

    pub fn render(&self, children: Vec<H::DomNode>) -> Option<H::DomNode> {
        match *self {
            VirtualNode::Host(ref node) => node.render(children),
            VirtualNode::Stateful(ref node) => node.render(children),
        }
    }
}
