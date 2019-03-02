use element::{Element, HostElement};

pub trait Component<H: HostElement>: Sized {
    type Props: Clone;
    type State;

    fn render(&self, props: &Self::Props, state: &Self::State) -> Element<H>;

    fn create(initial_props: &Self::Props) -> (Self, Self::State);
}
