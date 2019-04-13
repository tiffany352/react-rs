use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use react_rs::{DomNode, HostElement};

pub struct Widget<'a> {
    element: &'a WidgetElement,
    children: Vec<Widget<'a>>,
}

impl<'a> DomNode<'a> for Widget<'a> {
    type Widget = WidgetElement;

    fn new_dom_node(element: &'a WidgetElement, children: Vec<Self>) -> Self {
        Widget {
            element: element,
            children: children,
        }
    }
}

impl<'a> Widget<'a> {
    pub fn calc_size(&mut self, ctx: &mut Context) -> GameResult<Vector2<f32>> {
        match *self.element {
            WidgetElement::ListBox(ref props) => {
                let mut width = 0.0;
                let mut height = 0.0;
                for child in &mut self.children {
                    let size = child.calc_size(ctx)?;
                    if size.x > width {
                        width = size.x;
                    }
                    height += size.y;
                    height += props.padding;
                }
                Ok(Vector2::new(
                    width + props.padding * 2.0,
                    height + props.padding * 2.0,
                ))
            }
            WidgetElement::TextLabel(ref props) => {
                let text = Text::new(TextFragment::new(props.text.clone()).color(props.color));
                let (w, h) = text.dimensions(ctx);
                Ok(Vector2::new(w as f32, h as f32))
            }
        }
    }

    pub fn draw_inner(&mut self, ctx: &mut Context, pos: Point2<f32>) -> GameResult<Vector2<f32>> {
        match *self.element {
            WidgetElement::ListBox(ref props) => {
                let size = self.calc_size(ctx)?;
                let rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(pos.x, pos.y, size.x, size.y),
                    props.background_color,
                )?;
                graphics::draw(ctx, &rect, DrawParam::new())?;

                let mut cur = pos + Vector2::new(props.padding, props.padding);
                for child in &mut self.children {
                    let size = child.draw_inner(ctx, cur)?;
                    cur = cur + Vector2::new(0.0, size.y + props.padding);
                }
                Ok(size)
            }
            WidgetElement::TextLabel(ref props) => {
                let text = Text::new(TextFragment::new(props.text.clone()).color(props.color));
                graphics::draw(ctx, &text, (pos,))?;
                let (w, h) = text.dimensions(ctx);
                Ok(Vector2::new(w as f32, h as f32))
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_inner(ctx, Point2::new(0.0, 0.0))?;
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum WidgetElement {
    TextLabel(TextLabelElt),
    ListBox(ListBoxElt),
}

pub type Element = react_rs::Element<WidgetElement>;
pub use react_rs::Component;
pub type RenderContext<'a, Class> = react_rs::RenderContext<'a, WidgetElement, Class>;

impl HostElement for WidgetElement {}

#[derive(PartialEq)]
pub enum Font {
    Default,
}

#[derive(PartialEq)]
pub struct TextLabelElt {
    pub text: String,
    pub font: Font,
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextLabelElt {
    fn default() -> Self {
        TextLabelElt {
            text: "TextLabel".to_owned(),
            font: Font::Default,
            font_size: 12.0f32,
            color: Color::from_rgb(255, 255, 255),
        }
    }
}

impl Into<WidgetElement> for TextLabelElt {
    fn into(self) -> WidgetElement {
        WidgetElement::TextLabel(self)
    }
}

#[derive(PartialEq)]
pub struct ListBoxElt {
    pub padding: f32,
    pub background_color: Color,
}

impl Default for ListBoxElt {
    fn default() -> Self {
        ListBoxElt {
            padding: 4.0f32,
            background_color: Color::from_rgb(32, 32, 32),
        }
    }
}

impl Into<WidgetElement> for ListBoxElt {
    fn into(self) -> WidgetElement {
        WidgetElement::ListBox(self)
    }
}
