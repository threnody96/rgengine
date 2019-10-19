mod scene;
mod layer;

pub use self::scene::*;
pub use self::layer::*;

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::Ordering;
use ggez::{ Context };

pub struct NodeOption {
    z_index: i32,
    tags: Vec<String>
}

impl Default for NodeOption {

    fn default() -> Self {
        Self {
            z_index: 0,
            tags: Vec::new()
        }
    }

}

#[derive(Clone)]
pub struct NodeChild {
    id: String,
    z_index: i32,
    inner_z_index: u32,
    tags: Vec<String>
}

pub struct Node {
    id: String,
    children: RefCell<Vec<NodeChild>>,
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

    pub fn add_child(&self, delegate: Rc<dyn NodeDelegate>, option: NodeOption) {
        let inner_z_index = self.get_next_inner_z_index(option.z_index);
        let mut children = self.children.borrow_mut();
        children.push(NodeChild {
            id: delegate.id(),
            z_index: option.z_index,
            inner_z_index: inner_z_index,
            tags: option.tags.clone()
        });
        children.sort_by(|a, b| {
            let t = a.z_index.partial_cmp(&b.z_index).unwrap();
            if t != Ordering::Equal { return t; }
            a.inner_z_index.partial_cmp(&b.inner_z_index).unwrap()
        });
        delegate.node().add_referer(self.id.clone());
    }

    fn get_next_inner_z_index(&self, z_index: i32) -> u32 {
        let mut inner_z_index: u32 = 0;
        for child in &*self.children.borrow() {
            if child.z_index == z_index && child.inner_z_index > inner_z_index {
                inner_z_index = child.inner_z_index;
            }
        }
        inner_z_index + 1
    }

    fn delete_child(&self, id: String) {
        self.children.borrow_mut().retain(|c| { &c.id != &id });
    }

    fn add_referer(&self, id: String) {
        self.referers.borrow_mut().push(id);
    }

    fn destroy(&self) {
        let children = { self.children.borrow().clone() };
        let referers = { self.referers.borrow().clone() };
        for child in children {
            Node::get_by_id(child.id).destroy();
        }
        for referer_id in referers {
            Node::get_by_id(referer_id).delete_child(self.id.clone());
        }
        ::DIRECTOR.with(|d| {
            d.destroy_node(self.id.clone())
        });
    }

}

pub trait NodeDelegate {

    fn update(&self) { }

    fn update_all(&self) {
        self.update();
        let children = self.node().children.borrow().clone();
        for child in children {
            Node::get_delegate_by_id(child.id).update();
        }
    }

    fn render(&self, ctx: &mut Context) { }

    fn render_all(&self, ctx: &mut Context) {
        self.render(ctx);
        let children = self.node().children.borrow().clone();
        for child in children {
            Node::get_delegate_by_id(child.id).render(ctx);
        }
    }

    fn id(&self) -> String {
        format!("{:p}", self)
    }

    fn node(&self) -> Rc<Node> {
        Node::get_by_id(self.id())
    }

    fn add_child(&self, delegate: Rc<dyn NodeDelegate>, option: NodeOption) {
        delegate.check_add_child();
        self.node().add_child(delegate, option);
    }

    fn check_add_child(&self) { }

    fn destroy(&self) {
        self.node().destroy();
    }

}
