extern crate react_rs;

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
pub struct Widget<'a> {
    pub class: &'static str,
    // Just for debugging.
    pub element: &'a WidgetElement,
    // N.B. the type of the children array.
    pub children: Vec<Widget<'a>>,
}

/// A description of a widget, which will be reified into a virtual node.
#[derive(Debug, PartialEq)]
pub enum WidgetElement {
    Div(DivElement),
    Text(TextElement),
}

pub struct Callback(Option<Box<RefCell<dyn FnMut()>>>);

impl fmt::Debug for Callback {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl PartialEq for Callback {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Obligatory container element.
#[derive(Debug, PartialEq)]
pub struct DivElement {
    pub on_poke: Callback,
}

/// Text label element.
#[derive(Debug, PartialEq)]
pub struct TextElement {
    pub text: String,
}

impl<'a> DomNode<'a> for Widget<'a> {
    type Widget = WidgetElement;

    fn new_dom_node(element: &'a WidgetElement, children: Vec<Self>) -> Self {
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
impl HostElement for WidgetElement {}

// And now we construct an example "app" using our test gui framework
// from above.

pub struct Counter;

impl Component<WidgetElement> for Counter {
    type Props = ();
    type State = usize;

    fn create(_initial_props: &()) -> (Counter, usize) {
        (Counter, 0)
    }

    fn render(&self, ctx: RenderContext<WidgetElement, Self>) -> Element<WidgetElement> {
        let updater = ctx.updater;
        Element::new_host(
            WidgetElement::Text(TextElement {
                text: format!("{}", ctx.state),
            }),
            vec![Element::new_host(
                WidgetElement::Div(DivElement {
                    on_poke: Callback(Some(Box::new(RefCell::new(move || {
                        updater.set_state(|old_state| old_state + 1)
                    })))),
                }),
                vec![
                    Element::new_host(
                        WidgetElement::Text(TextElement {
                            text: "first child".to_owned(),
                        }),
                        vec![],
                    ),
                    Element::new_fragment(vec![
                        Element::new_host(
                            WidgetElement::Text(TextElement {
                                text: "fragment first".to_owned(),
                            }),
                            vec![],
                        ),
                        Element::new_host(
                            WidgetElement::Text(TextElement {
                                text: "fragment second".to_owned(),
                            }),
                            vec![]
                        ),
                    ]),
                ],
            )],
        )
    }
}

pub struct App;

impl Component<WidgetElement> for App {
    type Props = String;
    type State = ();

    fn create(_initial_props: &String) -> (App, ()) {
        (App, ())
    }

    fn render(&self, ctx: RenderContext<WidgetElement, Self>) -> Element<WidgetElement> {
        Element::new_host(
            WidgetElement::Div(DivElement {
                on_poke: Callback(None),
            }),
            vec![
                Element::new_host(
                    WidgetElement::Text(TextElement {
                        text: ctx.props.to_owned(),
                    }),
                    vec![],
                ),
                Element::new_stateful::<Counter>(()),
            ],
        )
    }
}

fn main() {
    let element = Element::new_stateful::<App>("App".to_owned());
    let mut tree = react_rs::VirtualTree::<WidgetElement>::mount(element);

    {
        let node = tree.render::<Widget>();
        println!("{:#?}", node);
        let poke: &RefCell<dyn FnMut()> = match node.as_ref() {
            Some(Widget { children, .. }) => match children[1].children[0].element {
                WidgetElement::Div(DivElement {
                    on_poke: Callback(Some(poke)),
                }) => &**poke,
                _ => panic!(),
            },
            _ => panic!(),
        };
        let mut borrow = poke.borrow_mut();
        let func: &mut dyn FnMut() = &mut *borrow;
        (*func)();
    }

    //let element = Element::new_stateful::<App>("App".to_owned());
    tree.flush();

    {
        let node = tree.render::<Widget>();
        println!("{:#?}", node);
    }

    let element = Element::new_stateful::<App>("App 2.0".to_owned());
    tree.update(element);

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
