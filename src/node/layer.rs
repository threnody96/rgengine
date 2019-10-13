use ::node::{ NodeDelegate };

pub struct Layer {}

impl NodeDelegate {

    fn render_self(&self) {
        println!("Layer");
    }

}