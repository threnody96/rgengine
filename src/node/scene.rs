use std::rc::Rc;
use std::any::Any;
use ::util::{ director, Size, AnchorPoint };
use ::node::{ Node, NodeDelegate, AddChildOption, NodeLike };
use ::node::layer::{ Layer };

pub trait Scene: NodeDelegate {

    fn add_layer(&self, node: Rc<Node<Layer>>, option: AddChildOption) {
        self.add_child(node, option);
    }

    fn update_scene(&self) {}

    fn render_scene(&self) {}

}

pub trait SceneLike: NodeLike {

    fn start_update(&self);

    fn start_render(&self);

}

impl <T> NodeDelegate for T where T: Scene {

    fn get_size(&self) -> Size {
        director(|d| d.get_resolution_size())
    }

    fn update(&self, _parent: Rc<dyn NodeLike>) { }

    fn render(&self, _parent: Rc<dyn NodeLike>) { }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        Some(AnchorPoint::new(0.0, 0.0))
    }

    fn before_add_child(&self, child: Rc<dyn NodeLike>) {
        panic!("Scene には add_child ではなく add_layer メソッドを使ってください");
    }

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

impl <T> SceneLike for Node<T> where T: Scene + Any {

    fn start_update(&self) {
        self.update_scene();
        self.update_children(self.node());
    }

    fn start_render(&self) {
        self.prepare_render_tree(&None);
        self.render_scene();
        self.render_children(self.node());
    }

}
