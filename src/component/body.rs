use ggez::{ Context, GameResult };
use ::controller::Input;

pub trait GameBody {

    fn update(&self, ctx: &mut Context, input: &Input) -> GameResult;

    fn draw(&self, ctx: &mut Context) -> GameResult;

}
