use ::node::{ NodeDelegate };

pub struct Scene {}

impl NodeDelegate for Scene {

    fn render_self(&self) {
        println!("Scene");
    }

}
