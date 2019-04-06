use super::HostNode;
use super::StatefulNodeWrapper;
use element::{Element, HostElement};
use std::iter::repeat_with;

pub enum VirtualNode<H: HostElement> {
    Host(HostNode<H>),
    Stateful(Box<dyn StatefulNodeWrapper<H>>),
}

impl<H> VirtualNode<H>
where
    H: HostElement,
{
    pub fn mount(element: Element<H>, nodes: &mut Vec<VirtualNode<H>>) -> usize {
        match element {
            Element::Host { element, children } => {
                let child_nodes = children
                    .into_iter()
                    .map(|child_element| VirtualNode::mount(child_element, nodes))
                    .collect::<Vec<usize>>();
                nodes.push(VirtualNode::Host(HostNode::mount(element, child_nodes)));
                nodes.len() - 1
            }
            Element::Stateful(node_creator) => {
                let mut node = node_creator.create_node();
                node.mount(nodes);
                nodes.push(VirtualNode::Stateful(node));
                nodes.len() - 1
            }
        }
    }

    pub fn update(
        node: VirtualNode<H>,
        element: Element<H>,
        old_nodes: &mut Vec<VirtualNode<H>>,
        new_nodes: &mut Vec<VirtualNode<H>>,
    ) -> usize {
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
            ) => {
                let child_nodes = children
                    .into_iter()
                    .rev()
                    .map(|index| {
                        assert!(index + 1 == old_nodes.len());
                        old_nodes.pop().unwrap()
                    })
                    .collect::<Vec<VirtualNode<H>>>();
                let child_indices = element_children
                    .into_iter()
                    .map(Some)
                    .chain(repeat_with(|| None))
                    .zip(
                        child_nodes
                            .into_iter()
                            .rev()
                            .map(Some)
                            .chain(repeat_with(|| None)),
                    )
                    .take_while(|(child_element, child_index)| {
                        child_element.is_some() && child_index.is_some()
                    })
                    .filter_map(
                        |(child_element, child_index)| match (child_element, child_index) {
                            (Some(child_element), Some(old_child)) => Some(VirtualNode::update(
                                old_child,
                                child_element,
                                old_nodes,
                                new_nodes,
                            )),
                            (Some(child_element), None) => {
                                Some(VirtualNode::mount(child_element, new_nodes))
                            }
                            (None, Some(old_child)) => {
                                VirtualNode::unmount(old_child, old_nodes);
                                None
                            }
                            (None, None) => None,
                        },
                    )
                    .collect::<Vec<usize>>();
                new_nodes.push(VirtualNode::Host(HostNode::mount(element, child_indices)));
                new_nodes.len() - 1
            }
            (VirtualNode::Stateful(mut node), element) => {
                match node.update(element, old_nodes, new_nodes) {
                    Ok(()) => {
                        new_nodes.push(VirtualNode::Stateful(node));
                        new_nodes.len() - 1
                    }
                    Err(element) => {
                        node.unmount(old_nodes);
                        VirtualNode::mount(element, new_nodes)
                    }
                }
            }
            (old_node, new_element) => {
                // If they're not compatible, we have to unmount and
                // remount.
                VirtualNode::unmount(old_node, old_nodes);
                VirtualNode::mount(new_element, new_nodes)
            }
        }
    }

    pub fn unmount(node: VirtualNode<H>, nodes: &mut Vec<VirtualNode<H>>) {
        match node {
            VirtualNode::Host(HostNode { children, .. }) => {
                for index in children.into_iter().rev() {
                    assert!(index + 1 == nodes.len());
                    let child = nodes.pop().unwrap();
                    VirtualNode::unmount(child, nodes);
                }
            }
            VirtualNode::Stateful(mut node) => node.unmount(nodes),
        }
    }

    pub fn render(&self, nodes: &[VirtualNode<H>]) -> Option<H::DomNode> {
        match *self {
            VirtualNode::Host(ref node) => node.render(nodes),
            VirtualNode::Stateful(ref node) => node.render(nodes),
        }
    }
}
