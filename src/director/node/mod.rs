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
        }
    }

}

// pub struct NodeEntry {
//     node: Rc<Node>,
//     delegate: Rc<dyn NodeDelegate>,
//     any_delegate: Rc<dyn Any>,
// }
//
// pub struct NodeDirector {
//     nodes: RefCell<HashMap<String, NodeEntry>>
// }
//
// impl NodeDirector {
//
//     pub fn new() -> Self {
//         Self {
//             nodes: RefCell::new(HashMap::new())
//         }
//     }
//
//     pub fn register_node<T>(&self, node: Rc<Node>, delegate: Rc<T>) where T: NodeDelegate + Any {
//         let mut nodes = self.nodes.borrow_mut();
//         nodes.insert(delegate.id(), NodeEntry {
//             node: node,
//             delegate: delegate.clone(),
//             any_delegate: delegate.clone()
//         });
//     }
//
//     pub fn get_node(&self, id: String) -> Rc<Node> {
//         let nodes = self.nodes.borrow();
//         nodes.get(&id).unwrap().node.clone()
//     }
//
//     pub fn get_delegate(&self, id: String) -> Rc<dyn NodeDelegate> {
//         let nodes = self.nodes.borrow();
//         nodes.get(&id).unwrap().delegate.clone()
//     }
//
//     pub fn get_delegate_by<T>(&self, id: String) -> Option<Rc<T>> where T: NodeDelegate + Any {
//         let nodes = self.nodes.borrow();
//         if let Ok(n) = nodes.get(&id).unwrap().any_delegate.clone().downcast::<T>() {
//             return Some(n);
//         }
//         None
//     }
//
//     pub fn destroy_node(&self, id: String) {
//         let mut nodes = self.nodes.borrow_mut();
//         nodes.remove(&id);
//     }
//
// }