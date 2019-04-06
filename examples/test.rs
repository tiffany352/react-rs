extern crate react_rs;

use react_rs::DomNode;
use react_rs::RenderContext;
use react_rs::{Component, Element, HostElement};

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
#[derive(Debug)]
pub enum WidgetElement {
    Div(DivElement),
    Text(TextElement),
}

/// Obligatory container element.
#[derive(Debug)]
pub struct DivElement {}

/// Text label element.
#[derive(Debug)]
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
        Element::new_host(
            WidgetElement::Text(TextElement {
                text: format!("{}", ctx.state),
            }),
            vec![],
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
            WidgetElement::Div(DivElement {}),
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
    let element = Element::new_stateful::<App>("First run".to_owned());
    let tree = react_rs::VirtualTree::<WidgetElement>::mount(element);

    {
        let node = tree.render::<Widget>();
        println!("{:#?}", node);
    }

    let element = Element::new_stateful::<App>("Second run".to_owned());
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
                    DivElement
                ),
                children: [
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "First run"
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
                        children: []
                    }
                ]
            }
        )
        Some(
            Widget {
                class: "div",
                element: Div(
                    DivElement
                ),
                children: [
                    Widget {
                        class: "text",
                        element: Text(
                            TextElement {
                                text: "Second run"
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
                        children: []
                    }
                ]
            }
        )
    */
}
