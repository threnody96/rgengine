use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ::node::{ Node, NodeDelegate, NodeId, NodeLike };
use ggez::{ Context };

pub struct NodeDirector {
    nodelikes: RefCell<HashMap<NodeId, Rc<dyn NodeLike>>>,
    anynodes: RefCell<HashMap<NodeId, Rc<dyn Any>>>
}

impl NodeDirector {

    pub fn new() -> Self {
        Self {
            nodelikes: RefCell::new(HashMap::new()),
            anynodes: RefCell::new(HashMap::new())
        }
    }

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        let mut nodelikes = self.nodelikes.borrow_mut();
        let mut anynodes = self.anynodes.borrow_mut();
        nodelikes.insert(node.id(), node.clone());
        anynodes.insert(node.id(), node.clone());
    }

    pub fn get_node<T>(&self, id: NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        let anynodes = self.anynodes.borrow();
        let node = anynodes.get(&id);
        if node.is_none() { return None; }
        if let Ok(n) = node.unwrap().clone().downcast::<Node<T>>() {
            return Some(n);
        }
        None
    }

    pub fn get_nodelike(&self, id: NodeId) -> Option<Rc<dyn NodeLike>> {
        let nodelikes = self.nodelikes.borrow();
        let node = nodelikes.get(&id);
        if node.is_none() { return None; }
        Some(node.unwrap().clone())
    }

    pub fn update(&self, id: NodeId) {
        let nodelikes = self.nodelikes.borrow();
        let node = nodelikes.get(&id);
        if node.is_none() { return; }
        node.unwrap().update();
    }

    pub fn render(&self, id: NodeId, ctx: &mut Context) {
        let nodelikes = self.nodelikes.borrow();
        let node = nodelikes.get(&id);
        if node.is_none() { return; }
        node.unwrap().render(ctx);
    }

    pub fn destroy(&self, id: NodeId) {
        let mut nodelikes = self.nodelikes.borrow_mut();
        let mut anynodes = self.anynodes.borrow_mut();
        anynodes.remove(&id);
        if let Some(node) = nodelikes.remove(&id) {
            for parent in node.get_parents() {
                self.get_nodelike(parent.clone()).unwrap().remove_child(id.clone());
            }
        }
    }

}

