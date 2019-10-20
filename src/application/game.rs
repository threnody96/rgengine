use std::rc::Rc;
use ::application::{ AppDelegate };
use ::node::{ Scene };
use ::util::{ Point, Size, director };
use ggez::{ Context, GameResult };
use ggez::event::{ EventHandler };
use ggez::timer::{ check_update_time, fps };
use ggez::graphics::{ DrawParam, Text, BLACK, WHITE, draw, clear, present };

pub struct Game {
    delegate: Rc<dyn AppDelegate>
}

impl Game {

    pub fn new(ctx: &mut Context, delegate: Rc<dyn AppDelegate>) -> Self {
        Self {
            delegate: delegate
        }
    }

    fn get_scene(&self) -> Rc<dyn Scene> {
        director(|d| d.get_scene())
    }

    fn draw_debug_message(&self, ctx: &mut Context) -> GameResult<()> {
        if director(|d| !d.get_display_stats()) { return Ok(()); }
        let fps_display = Text::new(format!("FPS: {}", fps(ctx).round() as u32));
        draw(
            ctx,
            &fps_display,
            (Point { x: 0.0, y: 0.0}, WHITE)
        )?;
        Ok(())
    }

}

impl EventHandler for Game {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while(check_update_time(ctx, self.delegate.fps())) {
            self.get_scene().update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, BLACK);
        self.get_scene().render(ctx);
        self.draw_debug_message(ctx);
        present(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        director(|d| {
            d.set_visible_size(Size {
                width: width,
                height: height
            })
        });
    }

}

