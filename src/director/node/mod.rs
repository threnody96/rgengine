use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ::node::{ Node, NodeDelegate, NodeId, NodeLike };
use ::util::{ Point };

pub struct NodeDirector {
    nodelikes: HashMap<NodeId, Rc<dyn NodeLike>>,
    anynodes: HashMap<NodeId, Rc<dyn Any>>,
    render_points: HashMap<NodeId, Point>,
}

impl NodeDirector {

    pub fn new() -> Self {
        Self {
            nodelikes: HashMap::new(),
            anynodes: HashMap::new(),
            render_points: HashMap::new(),
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

    pub fn get_nodelike(&self, id: &NodeId) -> Option<Rc<dyn NodeLike>> {
        let node = self.nodelikes.get(id);
        if node.is_none() { return None; }
        node.cloned()
    }

    pub fn set_render_point(&mut self, id: &NodeId, render_point: &Point) {
        self.render_points.insert(id.clone(), render_point.clone());
    }

    pub fn get_render_point(&self, id: &NodeId) -> Option<Point> {
        self.render_points.get(id).cloned()
    }

    pub fn clear_render_points(&mut self) {
        self.render_points = HashMap::new();
    }

    pub fn destroy(&mut self, id: &NodeId) {
        let nodelike = {
            self.anynodes.remove(id);
            self.nodelikes.remove(id)
        };
        if let Some(node) = nodelike {
            for parent in node.get_parents() {
                self.get_nodelike(&parent).unwrap().remove_child(id);
            }
            for child in node.get_children() {
                self.destroy(&child);
            }
        }
    }

}

