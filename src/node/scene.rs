use std::rc::Rc;
use std::any::Any;
use ::util::{ director, Size, AnchorPoint };
use ::node::{ Node, NodeDelegate, Layer, AddChildOption, NodeLike };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, node: Rc<Node<Layer>>, option: AddChildOption) {
        self.add_child(node, option);
    }

    fn update_scene(&self) {}

    fn render_scene(&self, parent: Option<Rc<dyn NodeLike>>) {}

}

pub trait SceneLike: NodeLike {

}

impl <T> NodeDelegate for T where T: Scene {

    fn get_size(&self) -> Size {
        director(|d| d.window_size())
    }

    fn update(&self) {
        self.update_scene();
    }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
        self.render_scene(parent);
    }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        Some(AnchorPoint { x: 0.0, y: 0.0 })
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
