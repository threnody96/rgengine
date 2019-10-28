use std::rc::Rc;
use std::any::Any;
use ::node::{ Node, NodeDelegate, Layer, AddChildOption, NodeLike };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, node: Rc<Node<Layer>>, option: AddChildOption) {
        self.add_child(node, option);
    }

    fn update_scene(&self) {}

    fn render_scene(&self) {}

}

pub trait SceneLike: NodeLike {

}

impl <T> NodeDelegate for T where T: Scene {

    fn update(&self) {
        self.update_scene();
    }

    fn render(&self) {
        self.render_scene();
    }

    fn before_add_child(&self) {
        panic!("Scene には add_child ではなく add_layer メソッドを使ってください");
    }

    fn before_be_added_child(&self) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

impl <T> SceneLike for Node<T> where T: Scene + Any {

}
