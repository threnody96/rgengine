mod scene;
mod layer;

pub use self::scene::*;
pub use self::layer::*;

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Node {
    id: String,
    children: RefCell<Vec<String>>,
    referers: RefCell<Vec<String>>
}

impl Node {

    pub fn create<F, T>(gen_delegate: F) -> Rc<T> where F: Fn() -> T, T: NodeDelegate + Any {
        let delegate = Rc::new(gen_delegate());
        let node = Rc::new(Self::new(delegate.clone()));
        ::DIRECTOR.with(|d| {
            d.register_node(node, delegate.clone());
        });
        delegate
    }

    fn new<T>(delegate: Rc<T>) -> Self where T: NodeDelegate {
        Self {
            id: delegate.id(),
            children: RefCell::new(Vec::new()),
            referers: RefCell::new(Vec::new())
        }
    }

    fn get_by_id(id: String) -> Rc<Node> {
        ::DIRECTOR.with(|d| {
            d.get_node(id)
        })
    }

    fn get_delegate_by_id(id: String) -> Rc<dyn NodeDelegate> {
        ::DIRECTOR.with(|d| {
            d.get_node_delegate(id)
        })
    }

    fn add_child(&self, delegate: Rc<dyn NodeDelegate>) {
        let mut children = self.children.borrow_mut();
        children.push(delegate.id());
        delegate.node().add_referer(self.id.clone());
    }

    fn delete_child(&self, id: String) {
        let mut children = self.children.borrow_mut();
        children.retain(|cid| cid != &id );
    }

    fn add_referer(&self, id: String) {
        let mut referers = self.referers.borrow_mut();
        referers.push(id);
    }

    fn destroy(&self) {
        for referer_id in self.referers.borrow().clone() {
            Node::get_by_id(referer_id).delete_child(self.id.clone());
        }
        ::DIRECTOR.with(|d| {
            d.destroy_node(self.id.clone())
        });
    }

}

pub trait NodeDelegate {

    fn render_self(&self);

    fn render(&self) {
        self.render_self();
        let children = self.node().children.borrow().clone();
        for child in children {
            Node::get_delegate_by_id(child).render();
        }
    }

    fn id(&self) -> String {
        format!("{:p}", self)
    }

    fn node(&self) -> Rc<Node> {
        Node::get_by_id(self.id())
    }

    fn add_child(&self, delegate: Rc<dyn NodeDelegate>) {
        self.node().add_child(delegate.clone());
    }

    fn destroy(&self) {
        self.node().destroy();
    }

}
