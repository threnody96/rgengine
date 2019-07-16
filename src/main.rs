extern crate image; 
extern crate ggez;
extern crate crypto;
extern crate rusqlite;
extern crate base64;

mod resource;
mod util;

use ggez::{
    Context,
    GameResult,
    graphics,
    event
};

use std::fs::File;
use std::io::Read;

struct MainState {
    image: graphics::Image
}

impl MainState {
    
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut file = File::open("hoge.png")?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        let a = image::load_from_memory(buf.as_slice())?;
        let b = a.to_rgba();
        let mut c = Vec::new();
        b.pixels()
            .into_iter()
            .for_each(|item| c.extend(item.data.to_vec()));
        let image = graphics::Image::from_rgba8(ctx, b.width() as u16, b.height() as u16, c.as_slice())?;
        Ok(
            MainState {
                image: image
            }
        )
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.image, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("drawing", "ggez");
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, state)
}
