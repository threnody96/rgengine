use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::any::Any;
use ::node::{ NodeChild, NodeDelegate, NodeId, NodeLike, AddChildOption };
use ::util::{ director };
use ::util::parameter::{ Point, AnchorPoint, Size };
use ::resource::{ Texture, Font, ResourceKey };
use sdl2::pixels::{ Color };

pub struct Node<T> where T: NodeDelegate + Any {
    delegate: T,
    position: RefCell<Point>,
    anchor_point: RefCell<AnchorPoint>,
    visible: RefCell<bool>,
    opacity: RefCell<u8>,
    rotation: RefCell<f64>,
    render_cache: RefCell<Option<ResourceKey>>,
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

    fn node(&self) -> Rc<dyn NodeLike> {
        self.delegate.node()
    }

    fn use_cache(&self) -> bool {
        self.delegate.use_cache()
    }

    fn get_cache(&self) -> Option<ResourceKey> {
        self.render_cache.borrow().clone()
    }

    fn set_cache(&self, key: Option<ResourceKey>) {
        self.render_cache.replace(key);
    }

    fn clear_cache(&self) {
        if let Some(cache_key) = self.get_cache() {
            director(|d| d.destroy_render_cache(&cache_key));
            self.set_cache(None);
        }
        self.clear_parents_cache();
    }

    fn get_size(&self) -> Size {
        self.delegate.get_size()
    }

    fn get_render_point(&self) -> Point {
        let p = self.get_position();
        let ap = self.get_anchor_point();
        let s = self.get_size();
        Point::new(
            p.x() - ((s.width() as f32 * ap.x()).round() as i32),
            p.y() - ((s.height() as f32 * ap.y()).round() as i32)
        )
    }

    fn render_texture(&self, texture: Rc<Texture>) {
        self.delegate.render_texture(texture);
    }

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color) {
        self.delegate.render_label(text, font, color);
    }

    fn update(&self, parent: Rc<dyn NodeLike>) {
        self.delegate.update(parent);
        self.update_children(self.node());
    }

    fn update_children(&self, parent: Rc<dyn NodeLike>) {
        for child in &*self.children.borrow() {
            let c = director(|d| d.get_nodelike(&child.id));
            c.update(parent.clone());
        }
    }

    fn render(&self, parent: Rc<dyn NodeLike>) {
        self.prepare_render_tree(&Some(parent.clone()));
        self.delegate.render(parent);
        self.render_children(self.node());
    }

    fn render_children(&self, parent: Rc<dyn NodeLike>) {
        for child in &*self.children.borrow() {
            let c = director(|d| d.get_nodelike(&child.id));
            c.render(parent.clone());
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
        let node = director(|d| d.get_nodelike(id));
        let pid = self.id();
        node.remove_parent(&pid)
    }

    fn set_position(&self, point: &Point) {
        self.position.replace(point.clone());
        self.clear_parents_cache();
    }

    fn get_position(&self) -> Point {
        self.position.borrow().clone()
    }

    fn set_anchor_point(&self, anchor_point: &AnchorPoint) {
        self.anchor_point.replace(anchor_point.clone());
        self.clear_parents_cache();
    }

    fn get_anchor_point(&self) -> AnchorPoint {
        if let Some(fa) = self.delegate.get_fixed_anchor_point() {
            return fa;
        }
        self.anchor_point.borrow().clone()
    }

    fn set_opacity(&self, opacity: u8) {
        self.opacity.replace(opacity);
        self.clear_cache();
    }

    fn get_opacity(&self) -> u8 {
        self.opacity.borrow().clone()
    }

    fn set_visible(&self, visible: bool) {
        self.visible.replace(visible);
        self.clear_cache();
    }

    fn get_visible(&self) -> bool {
        self.visible.borrow().clone()
    }

    fn set_rotation(&self, rotation: f64) {
        self.rotation.replace(rotation);
        self.clear_parents_cache();
    }

    fn get_rotation(&self) -> f64 {
        self.rotation.borrow().clone()
    }

    fn destroy(&self) {
        self.clear_cache();
        let id = self.id();
        for parent_id in self.get_parents() {
            let parent = director(|d| d.get_nodelike(&parent_id));
            parent.remove_child(&id);
        }
        for child_id in self.get_children() {
            let child = director(|d| d.get_nodelike(&child_id));
            child.destroy();
        }
        director(|d| d.destroy_node(&id));
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

    pub fn add_child<A>(&self, node: Rc<dyn NodeLike>, option: A)
    where A: Into<AddChildOption>
    {
        (self as &NodeLike).add_child(node, option.into());
    }

    pub fn set_position<A>(&self, position: A)
    where A: Into<Point>
    {
        (self as &NodeLike).set_position(&position.into())
    }

    pub fn set_anchor_point<A>(&self, anchor_point: A)
    where A: Into<AnchorPoint>
    {
        (self as &NodeLike).set_anchor_point(&anchor_point.into())
    }

    fn new(delegate: T) -> Self {
        Self {
            delegate: delegate,
            position: RefCell::new(Point::new(0, 0)),
            anchor_point: RefCell::new(AnchorPoint::default()),
            parents: RefCell::new(Vec::new()),
            opacity: RefCell::new(255),
            rotation: RefCell::new(0.0),
            visible: RefCell::new(true),
            render_cache: RefCell::new(None),
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

    fn clear_parents_cache(&self) {
        for parent_id in self.get_parents() {
            let parent = director(|d| d.get_nodelike(&parent_id));
            parent.clear_cache();
        }
    }

}

