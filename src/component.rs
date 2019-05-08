use element::{Element, HostElement};
use reconciler::StateUpdater;
use std::marker::PhantomData;

enum ChildUpdate {}

pub struct CreateContext<'a, H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    pub props: &'a Class::Props,
}

pub struct RenderContext<'a, H: HostElement, Class: Component<H>> {
    pub props: &'a Class::Props,
    pub state: &'a Class::State,
    pub updater: StateUpdater<H, Class>,
}

pub trait Component<H: HostElement>: Sized {
    type Props: PartialEq;
    type State;

    fn create(ctx: CreateContext<H, Self>) -> (Self, Self::State);

    fn update(&mut self, ctx: RenderContext<H, Self>);

    fn render<'a>(&'a self, ctx: RenderContext<H, Self>) -> Element<'a, H>;

    fn will_unmount(&mut self) {}

    fn get_derived_state_from_props(
        _next_props: &Self::Props,
        last_state: Self::State,
    ) -> Self::State {
        last_state
    }
}

pub struct Child<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    _phantom: PhantomData<(Class, H)>,
    key: usize,
}

impl<H, Class> Child<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    pub fn mount(
        context: &CreateContext<H, Class>,
        initial_props: Class::Props,
    ) -> Child<H, Class> {
        unimplemented!()
    }

    pub fn update(&mut self, context: &CreateContext<H, Class>, new_props: Class::Props) {
        unimplemented!()
    }
}
