use element::HostElement;
use reconciler::virtual_node::VirtualNode;

pub struct HostNode<H>
where
    H: HostElement,
{
    pub element: H,
    pub children: Vec<usize>,
}

impl<H> HostNode<H>
where
    H: HostElement,
{
    pub fn mount(element: H, children: Vec<usize>) -> HostNode<H> {
        HostNode {
            element: element,
            children: children,
        }
    }

    pub fn render(&self, nodes: &[VirtualNode<H>]) -> Option<H::DomNode> {
        let children = self
            .children
            .iter()
            .filter_map(|index| nodes[*index].render(nodes))
            .collect::<Vec<H::DomNode>>();

        Some(H::new_dom_node(self.element.clone(), children))
    }
}
