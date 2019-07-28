use std::rc::Rc;
use ggez::{ Context, GameResult };
use ::resource::Resource;

pub trait Component {
    
    fn draw(&self, ctx: &mut Context, rsc: Rc<Resource>) -> GameResult;

    fn update(&mut self, _ctx: &mut Context, _rsc: Rc<Resource>) -> GameResult {
        Ok(())
    }

    fn detach(&self, _ctx: &mut Context, _rsc: Rc<Resource>) -> GameResult {
        Ok(())
    }

}
