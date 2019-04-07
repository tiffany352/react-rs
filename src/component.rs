use element::{Element, HostElement};
use reconciler::StateUpdater;
use toolshed::Arena;

pub struct RenderContext<'a, 'arena, H: HostElement, Class: Component<H>> {
    pub props: &'a Class::Props,
    pub state: &'a Class::State,
    pub updater: StateUpdater<H, Class>,
    pub arena: &'arena Arena,
}

impl<'a, 'arena, H, Class> RenderContext<'a, 'arena, H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    pub fn new_host_element(
        &self,
        host: H,
        children: &[&'arena Element<'arena, H>],
    ) -> &'arena Element<'arena, H> {
        Element::new_host(self.arena, host, children)
    }

    pub fn new_stateful_element<OtherClass>(
        &self,
        props: OtherClass::Props,
    ) -> &'arena Element<'arena, H>
    where
        OtherClass: Component<H> + 'arena,
        OtherClass::Props: 'arena,
    {
        Element::new_stateful::<OtherClass>(self.arena, props)
    }
}

pub trait Component<H: HostElement>: Sized {
    type Props: Clone + Copy;
    type State;

    fn render<'arena>(&self, ctx: RenderContext<'_, 'arena, H, Self>)
        -> &'arena Element<'arena, H>;

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
