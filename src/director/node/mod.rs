use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;
use ::node::{ Node, NodeDelegate, NodeId, NodeLike };

pub struct NodeDirector {
    nodelikes: HashMap<NodeId, Rc<dyn NodeLike>>,
    anynodes: HashMap<NodeId, Rc<dyn Any>>,
}

impl NodeDirector {

    pub fn new() -> Self {
        Self {
            nodelikes: HashMap::new(),
            anynodes: HashMap::new(),
        }
    }

    pub fn register_node<T>(&mut self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.nodelikes.insert(node.id(), node.clone());
        self.anynodes.insert(node.id(), node.clone());
    }

    pub fn get_node<T>(&self, id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        let node = self.anynodes.get(id);
        if node.is_none() { return None; }
        if let Ok(n) = node.unwrap().clone().downcast::<Node<T>>() {
            return Some(n);
        }
        None
    }

    pub fn get_nodelike(&self, id: &NodeId) -> Rc<dyn NodeLike> {
        let node = self.nodelikes.get(id);
        node.cloned().unwrap()
    }

    pub fn destroy(&mut self, id: &NodeId) {
        self.anynodes.remove(id);
        self.nodelikes.remove(id);
    }

}

