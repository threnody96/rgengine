use std::rc::Rc;
use std::any::Any;
use ::util::{ director };
use ::util::parameter::{ Size, AnchorPoint };
use ::node::{ Node, NodeDelegate, NodeLike };
use ::node::layer::{ Layer };

pub trait Scene: NodeDelegate {

    fn update_scene(&self) {}

    fn render_scene(&self) {}

}

pub trait SceneLike: NodeLike {

    fn start_update(&self);

    fn start_render(&self);

}

impl <T> NodeDelegate for T where T: Scene {

    fn get_size(&self) -> Size {
        director::get_resolution_size()
    }

    fn update(&self) { }

    fn render(&self) { }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        Some(AnchorPoint::new(0.0, 0.0))
    }

    fn before_add_child(&self, child: Rc<dyn NodeLike>) {
        let id = child.id();
        if director::get_node::<Layer>(&id).is_none() {
            panic!("Scene に add_child できるのは Layer Node だけです");
        }
    }

    fn before_be_added_child(&self, _parent: Rc<dyn NodeLike>) {
        panic!("Scene は他 Node の子になることはできません");
    }

}

impl <T> SceneLike for Node<T> where T: Scene + Any {

    fn start_update(&self) {
        self.update_scene();
        self.update_children();
    }

    fn start_render(&self) {
        self.prepare_render_tree();
        self.render_scene();
        self.render_children();
    }

}
