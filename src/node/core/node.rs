use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::any::Any;
use ::node::{ NodeChild, NodeDelegate, NodeId, NodeLike, AddChildOption };
use ::util::{ director };
use ggez::{ Context };

pub struct Node<T> where T: NodeDelegate + Any {
    delegate: T,
    parents: RefCell<Vec<NodeId>>,
    children: RefCell<Vec<NodeChild>>
}

impl <T> Deref for Node<T> where T: NodeDelegate + Any {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }

}

impl <T> NodeLike for Node<T> where T: NodeDelegate + Any {

    fn id(&self) -> NodeId {
        self.delegate.id()
    }

    fn update(&self) {
        self.delegate.update();
    }

    fn render(&self, ctx: &mut Context) {
        self.delegate.render(ctx);
    }

    fn add_parent(&self, id: NodeId) {
        self.parents.borrow_mut().push(id);
    }

    fn remove_parent(&self, id: NodeId) {
        let mut next_parents = Vec::new();
        for parent in &*self.parents.borrow() {
            if &id != parent { next_parents.push(parent.clone()); }
        }
        self.parents.replace(next_parents);
    }

    fn get_parents(&self) -> Vec<NodeId> {
        self.parents.borrow().clone()
    }

    fn add_child(&self, node: Rc<NodeLike>, option: AddChildOption) {
        self.before_add_child();
        node.before_be_added_child();
        let inner_z_index = self.get_next_inner_z_index(option.z_index);
        let mut children = self.children.borrow_mut();
        children.push(NodeChild {
            id: node.id(),
            z_index: option.z_index,
            inner_z_index: inner_z_index,
            tag: option.tag.clone()
        });
        children.sort_by(|a, b| {
            let t = a.z_index.partial_cmp(&b.z_index).unwrap();
            if t != Ordering::Equal { return t; }
            a.inner_z_index.partial_cmp(&b.inner_z_index).unwrap()
        });
        node.add_parent(self.id());
    }

    fn remove_child(&self, id: NodeId) {
        let mut next_children = Vec::new();
        for child in &*self.children.borrow() {
            if id != child.id { next_children.push(child.clone()); }
        }
        self.children.replace(next_children);
        director(|d| {
            d.get_nodelike(id).unwrap().remove_parent(self.id());
        });
    }

}

impl <T> Node<T> where T: NodeDelegate + Any {

    pub fn create<C>(callback: C) -> Rc<Self> where C: Fn() -> T {
        director(|d| {
            let s = Rc::new(Self::new(callback()));
            d.register_node(s.clone());
            s
        })
    }

    fn new(delegate: T) -> Self {
        Self {
            delegate: delegate,
            parents: RefCell::new(Vec::new()),
            children: RefCell::new(Vec::new())
        }
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

}

