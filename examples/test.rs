extern crate react_rs;

use react_rs::toolshed::CopyCell;
use react_rs::DomNode;
use react_rs::RenderContext;
use react_rs::{Component, Element, HostElement};
use std::cell::RefCell;
use std::fmt;

// First, we need to define our "gui framework" to use. `react_rs` is
// generic over the host element type.

/// The thing that should be reified to - can be either a virtual node
/// to be reconciled, or directly passed to a renderer.
#[derive(Debug)]
pub struct Widget<'arena> {
    pub class: &'static str,
    // Just for debugging.
    pub element: &'arena WidgetElement<'arena>,
    // N.B. the type of the children array.
    pub children: Vec<Widget<'arena>>,
}

/// A description of a widget, which will be reified into a virtual node.
#[derive(Debug, Clone, Copy)]
pub enum WidgetElement<'arena> {
    Div(DivElement<'arena>),
    Text(TextElement<'arena>),
}

#[derive(Clone, Copy)]
pub struct Callback<'arena>(Option<&'arena dyn FnMut()>);

impl<'arena> fmt::Debug for Callback<'arena> {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// Obligatory container element.
#[derive(Debug, Clone, Copy)]
pub struct DivElement<'arena> {
    pub on_poke: Callback<'arena>,
}

/// Text label element.
#[derive(Debug, Clone, Copy)]
pub struct TextElement<'arena> {
    pub text: &'arena str,
}

impl<'arena> DomNode<'arena> for Widget<'arena> {
    type Widget = WidgetElement<'arena>;

    fn new_dom_node(element: &'arena WidgetElement, children: Vec<Self>) -> Self {
        Widget {
            class: match element {
                WidgetElement::Div(_) => "div",
                WidgetElement::Text(_) => "text",
            },
            element: element,
            children: children,
        }
    }
}

/// Called by the reifier when reifying elements into virtual nodes.
impl<'arena> HostElement for WidgetElement<'arena> {}

// And now we construct an example "app" using our test gui framework
// from above.

pub struct Counter;

impl<'arena> Component<WidgetElement<'arena>> for Counter {
    type Props = ();
    type State = usize;

    fn create(_initial_props: &()) -> (Counter, usize) {
        (Counter, 0)
    }

    fn render(
        &self,
        ctx: RenderContext<'_, 'arena, WidgetElement<'arena>, Self>,
    ) -> &'arena Element<'arena, WidgetElement<'arena>> {
        let updater = ctx.updater;
        ctx.new_host_element(
            WidgetElement::Text(TextElement {
                text: ctx.arena.alloc_string(format!("{}", ctx.state)),
            }),
            &[ctx.new_host_element(
                WidgetElement::Div(DivElement {
                    on_poke: Callback(Some(
                        ctx.arena
                            .alloc(move || updater.set_state(|old_state| old_state + 1)),
                    )),
                }),
                &[],
            )],
        )
    }
}

pub struct App;

impl<'arena> Component<WidgetElement<'arena>> for App {
    type Props = &'arena str;
    type State = ();

    fn create(_initial_props: Self::Props) -> (App, ()) {
        (App, ())
    }

    fn render(
        &self,
        ctx: RenderContext<'_, 'arena, WidgetElement<'arena>, Self>,
    ) -> &'arena Element<'arena, WidgetElement<'arena>> {
        ctx.new_host_element(
            WidgetElement::Div(DivElement {
                on_poke: Callback(None),
            }),
            &[
                ctx.new_host_element(
                    WidgetElement::Text(TextElement {
                        text: ctx.arena.alloc_str(ctx.props),
                    }),
                    &[],
                ),
                ctx.new_stateful_element::<Counter>(()),
            ],
        )
    }
}

fn main() {
    let tree = react_rs::VirtualTree::<WidgetElement>::mount(|arena| {
        Element::new_stateful::<App>(arena, "App")
    });

    {
        let node = tree.render::<Widget>();
        println!("{:#?}", node);
        let poke: &dyn FnMut() = match node.as_ref() {
            Some(Widget { children, .. }) => match children[1].children[0].element {
                WidgetElement::Div(DivElement {
                    on_poke: Callback(Some(poke)),
                }) => &**poke,
                _ => panic!(),
            },
            _ => panic!(),
        };
        (*poke)();
    }

    let element = Element::new_stateful::<App>("App".to_owned());
    let tree = tree.update(element);

    {
        let node = tree.render::<Widget>();
        println!("{:#?}", node);
    }

    tree.unmount();

    /*
        Some(
            Widget {
                class: "div",
                element: Div(
                    DivElement {
                        on_poke:
                    }
                ),
                children: [
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "App"
                            }
                        ),
                        children: []
                    },
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "0"
                            }
                        ),
                        children: [
                            Widget {
                                class: "div",
                                element: Div(
                                    DivElement {
                                        on_poke:
                                    }
                                ),
                                children: []
                            }
                        ]
                    }
                ]
            }
        )
        Some(
            Widget {
                class: "div",
                element: Div(
                    DivElement {
                        on_poke:
                    }
                ),
                children: [
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "1"
                            }
                        ),
                        children: [
                            Widget {
                                class: "div",
                                element: Div(
                                    DivElement {
                                        on_poke:
                                    }
                                ),
                                children: []
                            }
                        ]
                    },
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "App"
                            }
                        ),
                        children: []
                    }
                ]
            }
        )
    */
}
