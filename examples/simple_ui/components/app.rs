use ggez::graphics::Color;
use ui::*;

pub struct App;

impl Component<WidgetElement> for App {
    type Props = ();
    type State = ();

    fn create(_initial_props: &Self::Props) -> (Self, Self::State) {
        (App, ())
    }

    fn render(&self, _ctx: RenderContext<'_, App>) -> Element {
        Element::new_host(
            ListBoxElt {
                ..Default::default()
            },
            vec![
                Element::new_host(
                    TextLabelElt {
                        text: "Line 1".to_owned(),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::new_host(
                    TextLabelElt {
                        text: "Line 2".to_owned(),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::new_host(
                    TextLabelElt {
                        text: "Line 3".to_owned(),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::new_host(
                    ListBoxElt {
                        background_color: Color::from_rgb(32, 32, 96),
                        ..Default::default()
                    },
                    vec![Element::new_host(
                        TextLabelElt {
                            text: "the quick brown fox jumps over the lazy dog".to_owned(),
                            ..Default::default()
                        },
                        vec![],
                    )],
                ),
            ],
        )
    }
}
