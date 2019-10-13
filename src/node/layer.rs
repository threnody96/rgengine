use ::node::{ NodeDelegate };

pub struct Layer {}

impl NodeDelegate for Layer {

    fn render_self(&self) {
        println!("Layer");
    }

}