pub mod node;
pub mod application;

use std::rc::Rc;
use std::any::Any;
use self::node::{ NodeEntry, NodeDirector };
use self::application::{ ApplicationDerector };
use ::node::{ Node, NodeDelegate, Scene };
use ::application::{ AppDelegate, ResolutionPolicy, ResolutionSize };
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

    pub fn run_with_scene(&self, app_delegate: Rc<dyn AppDelegate>, scene: Rc<dyn Scene>) {
        self.application.run_with_scene(app_delegate, scene);
    }

    pub fn get_scene(&self) -> Rc<dyn Scene> {
        self.application.get_scene()
    }

    pub fn set_scene(&self, scene: Rc<dyn Scene>) {
        self.application.set_scene(scene);
    }

    pub fn get_visible_size(&self) -> Size {
        self.application.get_visible_size()
    }

    pub fn set_visible_size(&self, size: Size) {
        self.application.set_visible_size(size);
    }

    pub fn get_resolution_size(&self) -> ResolutionSize {
        self.application.get_resolution_size()
    }

    pub fn set_resolution_size(&self, size: Size, policy: ResolutionPolicy) {
        self.application.set_resolution_size(size, policy);
    }

    pub fn get_display_stats(&self) -> bool {
        self.application.get_display_stats()
    }

    pub fn set_display_stats(&self, display_stats: bool) {
        self.application.set_display_stats(display_stats);
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