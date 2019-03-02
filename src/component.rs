use element::{Element, HostElement};

pub trait Component<H: HostElement, Props = (), State = ()> {
    fn render(&self, props: &Props, state: &State, children: &[Element<H>]) -> Element<H>;
}
