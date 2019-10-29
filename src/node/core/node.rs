use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::any::Any;
use ::node::{ NodeChild, NodeDelegate, NodeId, NodeLike, AddChildOption };
use ::util::{ director, Point, AnchorPoint, Size };
use ::resource::{ RTexture, RFont };
use sdl2::pixels::{ Color };

pub struct Node<T> where T: NodeDelegate + Any {
    delegate: T,
    position: RefCell<Point>,
    anchor_point: RefCell<AnchorPoint>,
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

    fn get_size(&self) -> Size {
        self.delegate.get_size()
    }

    fn get_render_point(&self) -> Point {
        let p = self.get_position();
        let ap = self.get_anchor_point();
        let s = self.get_size();
        Point::new(
            p.x() - ((s.width as f32 * ap.x).round() as i32),
            p.y() - ((s.height as f32 * ap.y).round() as i32)
        )
    }

    fn render_texture(&self, parent: &Option<Rc<dyn NodeLike>>, texture: Rc<RTexture>) {
        self.delegate.render_texture(parent, texture);
    }

    fn render_label(&self, parent: &Option<Rc<dyn NodeLike>>, text: &str, font: Rc<RFont>, color: &Color) {
        self.delegate.render_label(parent, text, font, color);
    }

    fn update(&self) {
        self.delegate.update();
        for child in &*self.children.borrow() {
            let c = director(|d| d.get_nodelike(&child.id));
            if c.is_some() { c.unwrap().update(); }
        }
    }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
        self.delegate.render(parent);
        for child in &*self.children.borrow() {
            let id = self.id();
            let current = director(|d| d.get_nodelike(&id));
            let c = director(|d| d.get_nodelike(&child.id));
            if c.is_some() { c.unwrap().render(current); }
        }
    }

    fn add_parent(&self, id: &NodeId) {
        self.parents.borrow_mut().push(id.clone());
    }

    fn remove_parent(&self, id: &NodeId) {
        let mut next_parents = Vec::new();
        for parent in &*self.parents.borrow() {
            if id != parent { next_parents.push(parent.clone()); }
        }
        self.parents.replace(next_parents);
    }

    fn get_parents(&self) -> Vec<NodeId> {
        self.parents.borrow().clone()
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
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
        let id = self.id();
        node.add_parent(&id);
    }

    fn get_children(&self) -> Vec<NodeId> {
        let mut output: Vec<NodeId> = Vec::new();
        let children = self.children.borrow();
        for child in &*children {
            output.push(child.id.clone());
        }
        output
    }

    fn remove_child(&self, id: &NodeId) {
        let mut next_children = Vec::new();
        for child in &*self.children.borrow() {
            if id != &child.id { next_children.push(child.clone()); }
        }
        self.children.replace(next_children);
        let node = director(|d| {
            d.get_nodelike(id).unwrap()
        });
        let pid = self.id();
        node.remove_parent(&pid)
    }

    fn set_position(&self, point: &Point) {
        self.position.replace(point.clone());
    }

    fn get_position(&self) -> Point {
        self.position.borrow().clone()
    }

    fn set_anchor_point(&self, anchor_point: &AnchorPoint) {
        self.anchor_point.replace(anchor_point.clone());
    }

    fn get_anchor_point(&self) -> AnchorPoint {
        if let Some(fa) = self.delegate.get_fixed_anchor_point() {
            return fa;
        }
        self.anchor_point.borrow().clone()
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
            position: RefCell::new(Point::new(0, 0)),
            anchor_point: RefCell::new(AnchorPoint::default()),
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

