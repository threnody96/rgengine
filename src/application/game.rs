use std::rc::Rc;
use ::application::{ AppDelegate };
use ::util::{ Size };
use ggez::{ Context, GameResult };
use ggez::event::{ EventHandler };

pub struct Game {
    delegate: Rc<dyn AppDelegate>
}

impl Game {

    pub fn new(ctx: &mut Context, delegate: Rc<dyn AppDelegate>) -> Self {
        Self {
            delegate: delegate
        }
    }

}

impl EventHandler for Game {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        ::DIRECTOR.with(|d| {
            d.set_visible_size(Size {
                width: width,
                height: height
            })
        });
    }

}

