use element::{Element, HostElement};

pub trait Component<H: HostElement, Props = (), State = ()>: Sized {
    fn render(&self, props: &Props, state: &State) -> Element<H>;

    fn create(initial_props: &Props) -> (Self, State);
}
