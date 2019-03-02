extern crate react_rs;

use react_rs::{Component, Element, HostElement};

// First, we need to define our "gui framework" to use. `react_rs` is
// generic over the host element type.

/// The thing that should be reified to - can be either a virtual node
/// to be reconciled, or directly passed to a renderer.
#[derive(Debug)]
pub struct Widget {
    pub class: &'static str,
    // Just for debugging.
    pub element: WidgetElement,
    // N.B. the type of the children array.
    pub children: Vec<Widget>,
}

/// A description of a widget, which will be reified into a virtual node.
#[derive(Debug, Clone)]
pub enum WidgetElement {
    Div(DivElement),
    Text(TextElement),
}

/// Obligatory container element.
#[derive(Debug, Clone)]
pub struct DivElement {}

/// Text label element.
#[derive(Debug, Clone)]
pub struct TextElement {
    pub text: String,
}

/// Called by the reifier when reifying elements into virtual nodes.
impl HostElement for WidgetElement {
    type DomNode = Widget;

    fn new_dom_node(element: Self, children: Vec<Self::DomNode>) -> Self::DomNode {
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

// And now we construct an example "app" using our test gui framework
// from above.

pub struct Counter;

impl Component<WidgetElement> for Counter {
    type Props = ();
    type State = usize;

    fn create(_initial_props: &()) -> (Counter, usize) {
        (Counter, 0)
    }

    fn render(&self, _props: &(), state: &usize) -> Element<WidgetElement> {
        Element::new_host(
            WidgetElement::Text(TextElement {
                text: format!("{}", state),
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

    fn render(&self, props: &String, _state: &()) -> Element<WidgetElement> {
        Element::new_host(
            WidgetElement::Div(DivElement {}),
            vec![
                Element::new_host(
                    WidgetElement::Text(TextElement {
                        text: props.to_owned(),
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
        let node = tree.render();
        println!("{:#?}", node);
    }

    let element = Element::new_stateful::<App>("Second run".to_owned());
    let tree = tree.update(element);

    {
        let node = tree.render();
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
