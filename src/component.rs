use element::{Element, HostElement};
//use reconciler::StateUpdater;

pub struct RenderContext<'a, H: HostElement, Class: Component<H>> {
    pub props: &'a Class::Props,
    pub state: &'a Class::State,
    //pub updater: StateUpdater<H, Class>,
}

pub trait Component<H: HostElement>: Sized {
    type Props: Clone;
    type State;

    fn render(&self, ctx: RenderContext<H, Self>) -> Element<H>;

    fn create(initial_props: &Self::Props) -> (Self, Self::State);

    fn did_mount(&mut self) {}
    fn will_unmount(&mut self) {}

    fn get_derived_state_from_props(
        _next_props: &Self::Props,
        last_state: Self::State,
    ) -> Self::State {
        last_state
    }
}
