use std::rc::Rc;
use std::any::Any;
use ::node::{ Node, NodeDelegate, Layer };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, delegate: Rc<Layer>) {
        self.node().add_child(delegate);
    }

}

impl <T> NodeDelegate for T where T: Scene {

    fn render_self(&self) { }

    fn add_child(&self, delegate: Rc<dyn NodeDelegate>) {
        panic!("Scene には add_child ではなく add_layer メソッドを使ってください");
    }

    fn check_add_child(&self) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

pub struct BlankScene { }

impl Scene for BlankScene { }