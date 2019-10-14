pub mod node;
pub mod application;

use std::rc::Rc;
use std::any::Any;
use self::node::{ NodeEntry, NodeDirector };
use self::application::{ ApplicationDerector };
use ::node::{ Node, NodeDelegate, Scene };
use ::application::AppDelegate;
use ::util::{ Size };

pub struct Director {
    node: NodeDirector,
    application: ApplicationDerector
}

impl Director {

    pub fn new() -> Self {
        Self {
            node: NodeDirector::new(),
            application: ApplicationDerector::new()
        }
    }

    pub fn set_scene(&self, scene: Rc<dyn Scene>) {
        self.application.set_scene(scene);
    }

    pub fn set_visible_size(&self, size: Size) {
        self.application.set_visible_size(size);
    }

    pub fn register_node<T>(&self, node: Rc<Node>, delegate: Rc<T>) where T: NodeDelegate + Any {
        self.node.register_node(node, delegate);
    }

    pub fn get_node(&self, id: String) -> Rc<Node> {
        self.node.get_node(id)
    }

    pub fn get_node_delegate(&self, id: String) -> Rc<dyn NodeDelegate> {
        self.node.get_delegate(id)
    }

    pub fn destroy_node(&self, id: String) {
        self.node.destroy_node(id);
    }

}