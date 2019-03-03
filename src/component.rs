use element::{Element, HostElement};
use reconciler::StateUpdater;

pub trait Component<H: HostElement>: Sized {
    type Props: Clone;
    type State;

    fn render(
        &self,
        props: &Self::Props,
        state: &Self::State,
        state: StateUpdater<H, Self>,
    ) -> Element<H>;

    fn create(initial_props: &Self::Props) -> (Self, Self::State);

    fn did_mount(&mut self, state: StateUpdater<H, Self>) {}

    fn will_unmount(&mut self) {}

    fn get_derived_state_from_props(
        _next_props: &Self::Props,
        last_state: Self::State,
    ) -> Self::State {
        last_state
    }
}
