use std::rc::Rc;
use std::any::Any;
use ::node::{ Node, NodeDelegate, Layer, NodeOption };
use ggez::{ Context };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, delegate: Rc<Layer>, option: NodeOption) {
        self.node().add_child(delegate, option);
    }

}

impl <T> NodeDelegate for T where T: Scene {

    fn update(&self) { }

    fn render_self(&self, ctx: &mut Context) { }

    fn add_child(&self, delegate: Rc<dyn NodeDelegate>, option: NodeOption) {
        panic!("Scene には add_child ではなく add_layer メソッドを使ってください");
    }

    fn check_add_child(&self) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

