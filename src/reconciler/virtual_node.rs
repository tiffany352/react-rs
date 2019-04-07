use super::HostNode;
use super::StatefulNodeWrapper;
use element::DomNode;
use element::{Element, HostElement};
use reconciler::GenericStateUpdater;
use toolshed::Arena;

pub enum VirtualNode<H: HostElement> {
    Host(HostNode<H>),
    Stateful(Box<dyn StatefulNodeWrapper<H>>),
}

impl<H> VirtualNode<H>
where
    H: HostElement,
{
    pub fn mount<'arena>(
        arena: &'arena Arena,
        element: Element<'arena, H>,
        updater: GenericStateUpdater<H>,
    ) -> (VirtualNode<H>, &'arena [&'arena Element<'arena, H>]) {
        match element {
            Element::Host { element, children } => {
                (VirtualNode::Host(HostNode::mount(element)), children)
            }
            Element::Stateful(node_creator) => {
                let mut node = node_creator.create_node();
                let one_child = node.mount(arena, updater);
                (VirtualNode::Stateful(node), arena.alloc_slice(&[one_child]))
            }
        }
    }

    pub fn update<'arena>(
        arena: &'arena Arena,
        node: VirtualNode<H>,
        element: Element<'arena, H>,
        updater: GenericStateUpdater<H>,
    ) -> (VirtualNode<H>, &'arena [&'arena Element<'arena, H>]) {
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
                match node.update(arena, element, updater.clone()) {
                    Ok(one_child) => (VirtualNode::Stateful(node), arena.alloc_slice(&[one_child])),
                    Err(element) => {
                        node.unmount(updater.clone());
                        VirtualNode::mount(arena, element, updater)
                    }
                }
            }
            (old_node, new_element) => {
                // If they're not compatible, we have to unmount and
                // remount.
                VirtualNode::unmount(old_node, updater.clone());
                VirtualNode::mount(arena, new_element, updater)
            }
        }
    }

    pub fn unmount(node: VirtualNode<H>, updater: GenericStateUpdater<H>) {
        match node {
            VirtualNode::Host(_) => (),
            VirtualNode::Stateful(mut node) => node.unmount(updater),
        }
    }

    pub fn render<'a, Dom>(&'a self, mut children: Vec<Dom>) -> Option<Dom>
    where
        Dom: DomNode<'a, Widget = H>,
    {
        match *self {
            VirtualNode::Host(ref node) => node.render(children),
            VirtualNode::Stateful(_) => {
                assert!(children.len() <= 1);
                let one_child = children.pop();
                one_child
            }
        }
    }
}
