use component::Component;
use component::CreateContext;
use component::RenderContext;
use element::Element;
use element::HostElement;
use flat_tree::NodeChildren;
use reconciler::virtual_node::VirtualNodeBox;
use reconciler::GenericStateUpdater;
use reconciler::VirtualNode;
use std::any::Any;
use std::marker::PhantomData;

pub struct StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H>,
{
    component: Class,
    props: Class::Props,
    state: Option<Class::State>,
    children: NodeChildren<VirtualNodeBox<H>>,
    _phantom: PhantomData<H>,
}

impl<H, Class> StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    pub fn mount(initial_props: Class::Props) -> Self {
        let (component, state) = Class::create(CreateContext {
            props: &initial_props,
        });
        StatefulNode {
            component: component,
            props: initial_props,
            state: Some(state),
            children: NodeChildren::new(),
            _phantom: PhantomData,
        }
    }

    pub fn update_state<Func>(&mut self, func: Func, updater: GenericStateUpdater<H>) -> Element<H>
    where
        Func: FnOnce(Class::State) -> Class::State,
    {
        self.state = Some((func)(self.state.take().unwrap()));
        let element = self.component.render(RenderContext {
            props: &self.props,
            state: self.state.as_ref().unwrap(),
            updater: updater.specialize(),
        });
        element
    }
}

impl<H, Class> VirtualNode<H> for StatefulNode<H, Class>
where
    H: HostElement,
    Class: Component<H> + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_children(&self) -> &NodeChildren<VirtualNodeBox<H>> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut NodeChildren<VirtualNodeBox<H>> {
        &mut self.children
    }

    fn render<'a>(&self, updater: GenericStateUpdater<H>) -> Element<'a, H> {
        let element: Element<'a, H> = self.component.render(RenderContext {
            props: &self.props,
            state: self.state.as_ref().unwrap(),
            updater: updater.specialize(),
        });

        element
    }
}
