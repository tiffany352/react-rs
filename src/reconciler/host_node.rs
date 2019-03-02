use element::{Element, HostElement};
use reconciler::VirtualNode;

pub struct HostNode<H>
where
    H: HostElement,
{
    pub element: H,
    pub children: Vec<VirtualNode<H>>,
}

impl<H> HostNode<H>
where
    H: HostElement,
{
    pub fn mount(element: H, children: Vec<Element<H>>) -> HostNode<H> {
        HostNode {
            element: element,
            children: children
                .into_iter()
                .map(|elt| VirtualNode::mount(elt))
                .collect::<Vec<_>>(),
        }
    }

    pub fn render(&self) -> Option<H::DomNode> {
        let children = self
            .children
            .iter()
            .filter_map(|node| node.render())
            .collect::<Vec<H::DomNode>>();

        Some(H::new_dom_node(self.element.clone(), children))
    }
}
