use element::HostElement;
use reconciler::VirtualNode;

pub struct HostNode<H>
where
    H: HostElement,
{
    pub element: H,
    pub children: Vec<Box<dyn VirtualNode<H>>>,
}

impl<H> VirtualNode<H> for HostNode<H>
where
    H: HostElement,
{
    fn mount(&mut self) {
        for mut child in &mut self.children {
            child.mount();
        }
    }

    fn update(&mut self) {}

    fn unmount(&mut self) {}

    fn render(&self) -> Option<H::DomNode> {
        let children = self
            .children
            .iter()
            .filter_map(|node| node.render())
            .collect::<Vec<H::DomNode>>();

        Some(H::new_dom_node(self.element.clone(), children))
    }
}
