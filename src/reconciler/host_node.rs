use element::HostElement;
use flat_tree::NodeChildren;
use reconciler::virtual_node::VirtualNode;

pub struct HostNode<H>
where
    H: HostElement,
{
    pub element: H,
    pub children: NodeChildren<VirtualNode<H>>,
}

impl<H> HostNode<H>
where
    H: HostElement,
{
    pub fn mount(element: H) -> HostNode<H> {
        HostNode {
            element: element,
            children: NodeChildren::new(),
        }
    }

    pub fn render(&self, children: Vec<H::DomNode>) -> Option<H::DomNode> {
        Some(H::new_dom_node(self.element.clone(), children))
    }
}
