use super::HostNode;
use super::StatefulNodeWrapper;
use element::DomNode;
use element::{Element, HostElement};
use flat_tree::NodeChildren;
use reconciler::GenericStateUpdater;

pub enum VirtualNode<H: HostElement> {
    Host(HostNode<H>),
    Stateful(Box<dyn StatefulNodeWrapper<H>>),
    Fragment(NodeChildren<VirtualNode<H>>),
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
            Element::Fragment(children) => (VirtualNode::Fragment(NodeChildren::new()), children),
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
    ) -> (VirtualNode<H>, Option<Vec<Element<H>>>) {
        match (node, element) {
            (
                VirtualNode::Host(HostNode {
                    element: old_element,
                    children,
                }),
                Element::Host {
                    element: new_element,
                    children: element_children,
                },
            ) => {
                let should_update = old_element != new_element;
                let node = VirtualNode::Host(HostNode {
                    element: new_element,
                    children,
                });
                if should_update {
                    (node, Some(element_children))
                } else {
                    (node, None)
                }
            }
            (VirtualNode::Stateful(mut node), element) => {
                match node.update(element, updater.clone()) {
                    Ok(Some(element)) => (VirtualNode::Stateful(node), Some(vec![element])),
                    Ok(None) => (VirtualNode::Stateful(node), None),
                    Err(element) => {
                        node.unmount(updater.clone());
                        let (node, children) = VirtualNode::mount(element, updater);
                        (node, Some(children))
                    }
                }
            }
            (old_node, new_element) => {
                // If they're not compatible, we have to unmount and
                // remount.
                VirtualNode::unmount(old_node, updater.clone());
                let (node, children) = VirtualNode::mount(new_element, updater);
                (node, Some(children))
            }
        }
    }

    pub fn unmount(node: VirtualNode<H>, updater: GenericStateUpdater<H>) {
        match node {
            VirtualNode::Host(_) => (),
            VirtualNode::Stateful(mut node) => node.unmount(updater),
            VirtualNode::Fragment(_) => (),
        }
    }

    pub fn render<'a, Dom>(&'a self, children: Vec<Dom>) -> Vec<Dom>
    where
        Dom: DomNode<'a, Widget = H>,
    {
        match *self {
            VirtualNode::Host(ref node) => match node.render(children) {
                Some(dom) => vec![dom],
                None => vec![],
            },
            VirtualNode::Stateful(_) => children,
            VirtualNode::Fragment(_) => children,
        }
    }
}
