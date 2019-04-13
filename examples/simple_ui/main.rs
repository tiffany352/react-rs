extern crate ggez;
extern crate react_rs;

mod components;
mod ui;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use components::App;
use react_rs::VirtualTree;
use ui::{Element, Widget, WidgetElement};

struct MainState {
    ui_tree: VirtualTree<WidgetElement>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let element = Element::new_stateful::<App>(());
        Ok(MainState {
            ui_tree: VirtualTree::mount(element),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.ui_tree.flush();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let widget = self.ui_tree.render::<Widget>();

        if let Some(mut widget) = widget {
            widget.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
