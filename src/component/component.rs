use ggez::{ Context, GameResult };

pub trait Component {
    
    fn update(&self, ctx: &mut Context) -> GameResult;

    fn draw(&self, ctx: &mut Context) -> GameResult;

}
