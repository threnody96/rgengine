use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ::node::{ Node, NodeDelegate };

pub struct NodeEntry {
    node: Rc<Node>,
    delegate: Rc<dyn NodeDelegate>,
    any_delegate: Rc<dyn Any>,
}

pub struct NodeDirector {
    nodes: RefCell<HashMap<String, NodeEntry>>
}

impl NodeDirector {

    pub fn new() -> Self {
        Self {
            nodes: RefCell::new(HashMap::new())
        }
    }

    pub fn register_node<T>(&self, node: Rc<Node>, delegate: Rc<T>) where T: NodeDelegate + Any {
        let mut nodes = self.nodes.borrow_mut();
        nodes.insert(delegate.id(), NodeEntry {
            node: node,
            delegate: delegate.clone(),
            any_delegate: delegate.clone()
        });
    }

    pub fn get_node(&self, id: String) -> Rc<Node> {
        let nodes = self.nodes.borrow();
        nodes.get(&id).unwrap().node.clone()
    }

    pub fn get_delegate(&self, id: String) -> Rc<dyn NodeDelegate> {
        let nodes = self.nodes.borrow();
        nodes.get(&id).unwrap().delegate.clone()
    }

    pub fn get_delegate_by<T>(&self, id: String) -> Option<Rc<T>> where T: NodeDelegate + Any {
        let nodes = self.nodes.borrow();
        if let Ok(n) = nodes.get(&id).unwrap().any_delegate.clone().downcast::<T>() {
            return Some(n);
        }
        None
    }

    pub fn destroy_node(&self, id: String) {
        let mut nodes = self.nodes.borrow_mut();
        nodes.remove(&id);
    }

}