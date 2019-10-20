use std::rc::Rc;
use std::any::Any;
use ::node::{ Node, NodeDelegate, Layer, AddChildOption };
use ggez::{ Context };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, node: Rc<Node<Layer>>, option: AddChildOption) {
        self.add_child(node, option);
    }

}

impl <T> NodeDelegate for T where T: Scene {

    fn update(&self) { }

    fn render(&self, ctx: &mut Context) { }

    fn before_add_child(&self) {
        panic!("Scene には add_child ではなく add_layer メソッドを使ってください");
    }

    fn before_be_added_child(&self) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

