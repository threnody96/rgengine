use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::any::Any;
use ::node::{ NodeChild, NodeDelegate, NodeId, NodeLike, AddChildOption };
use ::action::{ ActionLike, ActionStatus };
use ::util::{ director, get_mouse_position };
use ::util::parameter::{ Point, AnchorPoint, Size, Rect };
use ::resource::{ Texture, Font, ResourceKey };
use sdl2::pixels::{ Color };

pub struct Node<T> where T: NodeDelegate + Any {
    delegate: T,
    additive_blend: RefCell<bool>,
    absolute_position: RefCell<Point>,
    position: RefCell<Point>,
    anchor_point: RefCell<AnchorPoint>,
    visible: RefCell<bool>,
    opacity: RefCell<u8>,
    rotation: RefCell<f64>,
    scale: RefCell<f64>,
    render_cache: RefCell<Option<ResourceKey>>,
    parent: RefCell<Option<NodeId>>,
    children: RefCell<Vec<NodeChild>>,
    actions: RefCell<Vec<Rc<dyn ActionLike>>>,
    next_actions: RefCell<Vec<Rc<dyn ActionLike>>>
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

    fn before_add_child(&self, child: Rc<dyn NodeLike>) {
        self.delegate.before_add_child(child);
    }

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>) {
        self.delegate.before_be_added_child(parent);
    }

    fn clear_cache(&self) {
        if let Some(cache_key) = self.get_cache() {
            director(|d| d.destroy_render_cache(&cache_key));
            self.set_cache(None);
        }
        self.clear_parent_cache();
    }

    fn get_size(&self) -> Size {
        self.delegate.get_size()
    }

    fn get_scaled_size(&self) -> Size {
        let size = self.get_size();
        let scale = self.get_scale();
        Size::new((size.width() as f64 * scale) as u32, (size.height() as f64 * scale) as u32)
    }

    fn get_render_rect(&self) -> Rect {
        self.generate_render_rect(&self.get_position())
    }

    fn get_absolute_render_rect(&self) -> Rect {
        self.generate_render_rect(&self.get_absolute_position())
    }

    fn render_texture(&self, texture: Rc<Texture>) {
        self.delegate.render_texture(texture);
    }

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color) {
        self.delegate.render_label(text, font, color);
    }

    fn update(&self) {
        self.delegate.update();
        self.restore_next_action();
        for action in self.actions.borrow().iter() {
            action.run(self.node(), None);
        }
        self.remove_finished_actions();
        self.update_children();
    }

    fn update_children(&self) {
        for child in self.get_children() {
            child.update();
        }
    }

    fn render(&self) {
        self.prepare_render_tree();
        self.delegate.render();
        self.render_children();
    }

    fn render_children(&self) {
        for child in self.get_children() {
            child.render();
        }
    }

    fn set_parent(&self, id: &NodeId) {
        if self.get_parent().is_some() {
            panic!("既に親が存在する node です");
        }
        self.parent.replace(Some(id.clone()));
    }

    fn remove_parent(&self) {
        self.parent.replace(None);
        self.absolute_position.replace(self.get_position());
    }

    fn get_parent(&self) -> Option<Rc<dyn NodeLike>> {
        if let Some(id) = self.parent.borrow().clone() {
            Some(director(|d| d.get_nodelike(&id)))
        } else {
            None
        }
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.before_add_child(node.clone());
        node.before_be_added_child(self.node());
        let inner_z_index = self.get_next_inner_z_index(option.z_index);
        let mut children = self.children.borrow_mut();
        children.push(NodeChild {
            id: node.id(),
            z_index: option.z_index,
            inner_z_index: inner_z_index,
        });
        children.sort_by(|a, b| {
            let t = a.z_index.partial_cmp(&b.z_index).unwrap();
            if t != Ordering::Equal { return t; }
            a.inner_z_index.partial_cmp(&b.inner_z_index).unwrap()
        });
        node.set_parent(&self.id());
        node.update_absolute_position();
    }

    fn get_children(&self) -> Vec<Rc<dyn NodeLike>> {
        let mut output: Vec<Rc<dyn NodeLike>> = Vec::new();
        let children = self.children.borrow();
        for child in &*children {
            output.push(director(|d| d.get_nodelike(&child.id)));
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
        node.remove_parent()
    }

    fn set_position(&self, point: &Point) {
        self.position.replace(point.clone());
        self.update_absolute_position();
        self.clear_parent_cache();
    }

    fn get_position(&self) -> Point {
        self.position.borrow().clone()
    }

    fn update_absolute_position(&self) {
        let position = self.get_position();
        let parent_position = if let Some(parent) = self.get_parent() {
            let r = parent.get_absolute_render_rect();
            Point::new(r.x(), r.y())
        } else {
            Point::new(0, 0)
        };
        self.absolute_position.replace(Point::new(
            parent_position.x() + position.x(),
            parent_position.y() + position.y()
        ));
        for child in self.get_children() {
            child.update_absolute_position();
        }
    }

    fn get_absolute_position(&self) -> Point {
        self.absolute_position.borrow().clone()
    }

    fn set_anchor_point(&self, anchor_point: &AnchorPoint) {
        self.anchor_point.replace(anchor_point.clone());
        self.clear_parent_cache();
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

    fn set_scale(&self, scale: f64) {
        let s = if scale > 0.0 { scale } else { 0.0 };
        self.scale.replace(s);
    }

    fn get_scale(&self) -> f64 {
        self.scale.borrow().clone()
    }

    fn set_rotation(&self, rotation: f64) {
        let mut r = rotation % 360.0;
        if r < 0.0 { r += 360.0 }
        self.rotation.replace(r);
        self.clear_parent_cache();
    }

    fn get_rotation(&self) -> f64 {
        self.rotation.borrow().clone()
    }

    fn is_additive_blend(&self) -> bool {
        self.additive_blend.borrow().clone()
    }

    fn set_additive_blend(&self, additive_blend: bool) {
        self.additive_blend.replace(additive_blend);
    }

    fn is_mouse_hover(&self) -> bool {
        let p = get_mouse_position();
        let ap = self.get_absolute_render_rect();
        ap.x() <= p.x() && ap.x() + (ap.width() as i32) >= p.x() &&
        ap.y() <= p.y() && ap.y() + (ap.height() as i32) >= p.y()
    }

    fn is_conflict(&self, other: Rc<dyn NodeLike>) -> bool {
        let (r1, r2) = (self.get_absolute_render_rect(), other.get_absolute_render_rect());
        r1.has_intersection(*r2)
    }

    fn run_action(&self, action: Rc<dyn ActionLike>) {
        self.next_actions.borrow_mut().push(action);
    }

    fn destroy(&self) {
        self.clear_cache();
        let id = self.id();
        if let Some(parent) = self.get_parent() {
            parent.remove_child(&id);
        }
        for child in self.get_children() {
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
            additive_blend: RefCell::new(false),
            absolute_position: RefCell::new(Point::new(0, 0)),
            position: RefCell::new(Point::new(0, 0)),
            anchor_point: RefCell::new(AnchorPoint::default()),
            parent: RefCell::new(None),
            opacity: RefCell::new(255),
            rotation: RefCell::new(0.0),
            visible: RefCell::new(true),
            scale: RefCell::new(1.0),
            render_cache: RefCell::new(None),
            children: RefCell::new(Vec::new()),
            actions: RefCell::new(Vec::new()),
            next_actions: RefCell::new(Vec::new())
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

    fn clear_parent_cache(&self) {
        if let Some(parent) = self.get_parent() {
            parent.clear_cache();
        }
    }

    fn generate_render_rect(&self, position: &Point) -> Rect {
        let ap = self.get_anchor_point();
        let s = self.get_scaled_size();
        Rect::new(
            position.x() - ((s.width() as f32 * ap.x()).round() as i32),
            position.y() - ((s.height() as f32 * ap.y()).round() as i32),
            s.width(),
            s.height()
        )
    }

    fn remove_finished_actions(&self) {
        let mut next_actions: Vec<Rc<dyn ActionLike>> = Vec::new();
        for action in self.actions.borrow().iter() {
            if action.get_status() != ActionStatus::Finish {
                next_actions.push(action.clone());
            }
        }
        self.actions.replace(next_actions);
    }

    fn restore_next_action(&self) {
        let mut actions = self.actions.borrow_mut();
        let mut next_actions = self.next_actions.borrow_mut();
        actions.append(&mut next_actions);
    }

}

