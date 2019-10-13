pub mod node;
pub mod application;

use std::rc::Rc;
use std::any::Any;
use self::node::{ NodeEntry, NodeDirector };
use ::node::{ Node, NodeDelegate };

pub struct Director {
    node: NodeDirector
}

impl Director {

    pub fn new() -> Self {
        Self {
            node: NodeDirector::new()
        }
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