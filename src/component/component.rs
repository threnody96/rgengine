use std::rc::Rc;
use ggez::{ Context, GameResult };
use ::resource::Resource;

pub trait Component {
    
    fn update(&mut self, ctx: &mut Context, rsc: Rc<Resource>) -> GameResult;

    fn draw(&self, ctx: &mut Context, rsc: Rc<Resource>) -> GameResult;

    fn detach(&self, _ctx: &mut Context, _rsc: Rc<Resource>) -> GameResult {
        Ok(())
    }

}
