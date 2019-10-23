use std::rc::Rc;
use ::application::{ AppDelegate };
use ::node::{ SceneLike };
use ::util::{ Point, Size, director, context };
use ggez::{ Context, GameResult };
use ggez::event::{ EventHandler };
use ggez::timer::{ check_update_time, fps };
use ggez::graphics::{ DrawParam, Text, BLACK, WHITE, draw, clear, present };

pub struct Game {
    delegate: Rc<dyn AppDelegate>
}

impl Game {

    pub fn new(delegate: Rc<dyn AppDelegate>) -> Self {
        Self {
            delegate: delegate
        }
    }

    fn get_scene(&self) -> Rc<dyn SceneLike> {
        director(|d| d.get_scene())
    }

    fn draw_debug_message(&self) -> GameResult<()> {
        if director(|d| !d.get_display_stats()) { return Ok(()); }
        context(|ctx| {
            let fps_display = Text::new(format!("FPS: {}", fps(ctx).round() as u32));
            draw(
                ctx,
                &fps_display,
                (Point { x: 0.0, y: 0.0}, WHITE)
            )
        })?;
        Ok(())
    }

}

impl EventHandler for Game {

    fn update(&mut self) -> GameResult<()> {
        while { context(|ctx| check_update_time(ctx, self.delegate.fps())) } {
            self.get_scene().update();
        }
        Ok(())
    }

    fn draw(&mut self) -> GameResult<()> {
        context(|ctx| clear(ctx, BLACK));
        self.get_scene().render();
        self.draw_debug_message();
        context(|ctx| present(ctx))
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        director(|d| {
            d.set_visible_size(Size {
                width: width,
                height: height
            })
        });
    }

}

