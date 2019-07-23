use std::rc::Rc;
use ggez::{ Context, GameResult };
use ::controller::Input;

pub trait GameBody {

    fn update(&self, ctx: &mut Context) -> GameResult;

    fn draw(&self, ctx: &mut Context) -> GameResult;

    fn input(&self) -> Rc<Input>;

}
