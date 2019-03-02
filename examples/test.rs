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

/// Called by the reifier when reifying elements into virtual nodes.
impl HostElement for WidgetElement {
    type VirtualNode = Widget;

    fn new_virtual_node(element: Self, children: Vec<Self::VirtualNode>) -> Self::VirtualNode {
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

impl Component<WidgetElement, (), usize> for Counter {
    fn render(
        &self,
        _props: &(),
        state: &usize,
        _children: &[Element<WidgetElement>],
    ) -> Element<WidgetElement> {
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
    fn render(
        &self,
        _props: &(),
        _state: &(),
        _children: &[Element<WidgetElement>],
    ) -> Element<WidgetElement> {
        Element::new_host(
            WidgetElement::Div(DivElement {}),
            vec![Element::new_stateful(Counter, (), vec![])],
        )
    }
}

fn main() {
    let app = App;
    let element = Element::new_stateful(app, (), vec![]);
    let node = element.reify();
    println!("{:#?}", node);
    /*
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
                            text: "0"
                        }
                    ),
                    children: []
                }
            ]
        }
    */
}
