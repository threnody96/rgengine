use ::node::{ NodeDelegate };
use ggez::{ Context };

pub struct Layer {}

impl NodeDelegate for Layer {

    fn update(&self) { }

    fn render_self(&self, ctx: &mut Context) {
        println!("Layer");
    }

}