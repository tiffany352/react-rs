extern crate react_rs;

use react_rs::{Component, Element, ElementBox, HostElement, DivProps};

pub struct Counter;

impl Component<(), usize> for Counter {
    fn render(&self, _props: &(), _state: &usize) -> ElementBox {
        Element::new_host(HostElement::Div(DivProps::new()))
    }
}

pub struct App;

impl Component for App {
    fn render(&self, _props: &(), _state: &()) -> ElementBox {
        Element::new_host(HostElement::Div(DivProps {
            children: vec![
                Element::new_stateful(Counter, ())
            ]
        }))
    }
}

fn main() {
    let app = App;
    let _element = Element::new_stateful(app, ());
}
