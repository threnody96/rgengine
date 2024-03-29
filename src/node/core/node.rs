use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::any::Any;
use ::node::{ NodeChild, NodeDelegate, NodeId, NodeLike, AddChildOption, ConflictType, RunActionOption };
use ::action::{ ActionLike, ActionStatus };
use ::util::{ director, get_mouse_position };
use ::util::parameter::{ Point, AnchorPoint, Size, Rect, Circle, Color, Opacity, Scale, Rotation };
use ::resource::{ Texture, Font, ResourceKey };

pub struct Node<T> where T: NodeDelegate + Any {
    delegate: T,
    additive_blend: RefCell<bool>,
    absolute_position: RefCell<Point>,
    position: RefCell<Point>,
    anchor_point: RefCell<AnchorPoint>,
    visible: RefCell<bool>,
    opacity: RefCell<Opacity>,
    rotation: RefCell<Rotation>,
    scale: RefCell<Scale>,
    render_cache: RefCell<Option<ResourceKey>>,
    parent: RefCell<Option<NodeId>>,
    children: RefCell<Vec<NodeChild>>,
    child_map: RefCell<HashMap<String, NodeId>>,
    conflict_type: RefCell<ConflictType>,
    actions: RefCell<Vec<Rc<dyn ActionLike>>>,
    action_map: RefCell<HashMap<String, Rc<dyn ActionLike>>>,
    next_actions: RefCell<Vec<Rc<dyn ActionLike>>>
}

impl <T> Deref for Node<T> where T: NodeDelegate + Any {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }

}

impl <T> NodeLike for Node<T> where T: NodeDelegate + Any {

    fn inner_id(&self) -> NodeId {
        self.delegate.id()
    }

    fn inner_node(&self) -> Rc<dyn NodeLike> {
        self.delegate.node()
    }

    fn inner_use_cache(&self) -> bool {
        self.delegate.use_cache()
    }

    fn inner_get_cache(&self) -> Option<ResourceKey> {
        self.render_cache.borrow().clone()
    }

    fn inner_set_cache(&self, key: Option<ResourceKey>) {
        self.render_cache.replace(key);
    }

    fn inner_clear_cache(&self) {
        if let Some(cache_key) = self.inner_get_cache() {
            director::destroy_render_cache(&cache_key);
            self.inner_set_cache(None);
        }
        self.clear_parent_cache();
    }

    fn inner_get_size(&self) -> Size {
        self.delegate.get_size()
    }

    fn inner_get_scaled_size(&self) -> Size {
        let size = self.get_size();
        let scale = self.get_scale();
        Size::new((size.width() as f64 * *scale) as u32, (size.height() as f64 * *scale) as u32)
    }

    fn inner_get_render_rect(&self) -> Rect {
        self.generate_render_rect(&self.get_position())
    }

    fn inner_get_absolute_render_rect(&self) -> Rect {
        self.generate_render_rect(&self.inner_get_absolute_position())
    }

    fn inner_update(&self) {
        self.delegate.update();
        self.restore_next_action();
        for action in self.actions.borrow().iter() {
            action.run(self.node(), None);
        }
        self.remove_finished_actions();
        self.inner_update_children();
    }

    fn inner_update_children(&self) {
        for child in self.inner_get_children() {
            child.inner_update();
        }
    }

    fn inner_prepare_render_tree(&self) {
        director::prepare_render_tree(self.node());
    }

    fn inner_render(&self) {
        self.inner_prepare_render_tree();
        self.delegate.render();
        self.inner_render_children();
    }

    fn inner_render_children(&self) {
        for child in self.inner_get_children() {
            child.inner_render();
        }
    }

    fn inner_set_parent(&self, id: NodeId) {
        if self.inner_get_parent().is_some() {
            panic!("既に親が存在する node です");
        }
        self.parent.replace(Some(id));
    }

    fn inner_remove_parent(&self) {
        self.parent.replace(None);
        self.absolute_position.replace(self.get_position());
    }

    fn inner_get_parent_id(&self) -> Option<NodeId> {
        self.parent.borrow().clone()
    }

    fn inner_get_parent(&self) -> Option<Rc<dyn NodeLike>> {
        if let Some(id) = self.inner_get_parent_id() {
            Some(director::get_nodelike(&id))
        } else {
            None
        }
    }

    fn inner_before_add_child(&self, child: Rc<dyn NodeLike>) {
        self.delegate.before_add_child(child);
    }

    fn inner_before_be_added_child(&self, parent: Rc<dyn NodeLike>) {
        self.delegate.before_be_added_child(parent);
    }

    fn inner_add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.inner_before_add_child(node.clone());
        node.inner_before_be_added_child(self.node());
        let inner_z_index = self.get_next_inner_z_index(option.z_index);
        let mut children = self.children.borrow_mut();
        children.push(NodeChild {
            id: node.inner_id(),
            z_index: option.z_index,
            inner_z_index: inner_z_index,
        });
        children.sort_by(|a, b| {
            let t = a.z_index.partial_cmp(&b.z_index).unwrap();
            if t != Ordering::Equal { return t; }
            a.inner_z_index.partial_cmp(&b.inner_z_index).unwrap()
        });
        if let Some(name) = option.name.clone() {
            self.child_map.borrow_mut().insert(name, node.inner_id());
        }
        node.inner_set_parent(self.id());
        node.inner_update_absolute_position();
    }

    fn inner_get_children_ids(&self) -> Vec<NodeId> {
        let mut output: Vec<NodeId> = Vec::new();
        for child in self.children.borrow().iter() {
            output.push(child.id.clone());
        }
        output
    }

    fn inner_get_child_id(&self, name: &str) -> Option<NodeId> {
        self.child_map.borrow().get(name).cloned()
    }

    fn inner_get_children(&self) -> Vec<Rc<dyn NodeLike>> {
        let mut output: Vec<Rc<dyn NodeLike>> = Vec::new();
        for child_id in self.inner_get_children_ids() {
            output.push(director::get_nodelike(&child_id));
        }
        output
    }

    fn inner_remove_child(&self, id: NodeId) {
        let mut next_children = Vec::new();
        for child in self.children.borrow().iter() {
            if id != child.id { next_children.push(child.clone()); }
        }
        self.children.replace(next_children);
        let mut next_child_map: HashMap<String, NodeId> = HashMap::new();
        for (name, child_id) in self.child_map.borrow().iter() {
            if &id != child_id { next_child_map.insert(name.to_string(), child_id.clone()); }
        }
        self.child_map.replace(next_child_map);
        let node = director::get_nodelike(&id);
        node.inner_remove_parent();

    }

    fn inner_set_position(&self, point: Point) {
        self.position.replace(point);
        self.inner_update_absolute_position();
        self.clear_parent_cache();
    }

    fn inner_get_position(&self) -> Point {
        self.position.borrow().clone()
    }

    fn inner_update_absolute_position(&self) {
        let position = self.get_position();
        let parent_position = if let Some(parent) = self.inner_get_parent() {
            let r = parent.inner_get_absolute_render_rect();
            Point::new(r.x(), r.y())
        } else {
            Point::new(0, 0)
        };
        self.absolute_position.replace(Point::new(
            parent_position.x() + position.x(),
            parent_position.y() + position.y()
        ));
        for child in self.inner_get_children() {
            child.inner_update_absolute_position();
        }
    }

    fn inner_get_absolute_position(&self) -> Point {
        self.absolute_position.borrow().clone()
    }

    fn inner_set_anchor_point(&self, anchor_point: AnchorPoint) {
        self.anchor_point.replace(anchor_point);
        self.clear_parent_cache();
    }

    fn inner_get_anchor_point(&self) -> AnchorPoint {
        if let Some(fa) = self.delegate.get_fixed_anchor_point() {
            return fa;
        }
        self.anchor_point.borrow().clone()
    }

    fn inner_set_opacity(&self, opacity: Opacity) {
        self.opacity.replace(opacity.clone());
        self.clear_cache();
    }

    fn inner_get_opacity(&self) -> Opacity {
        self.opacity.borrow().clone()
    }

    fn inner_set_visible(&self, visible: bool) {
        self.visible.replace(visible);
        self.clear_cache();
    }

    fn inner_get_visible(&self) -> bool {
        self.visible.borrow().clone()
    }

    fn inner_set_scale(&self, scale: Scale) {
        self.scale.replace(scale);
    }

    fn inner_get_scale(&self) -> Scale {
        self.scale.borrow().clone()
    }

    fn inner_set_rotation(&self, rotation: Rotation) {
        self.rotation.replace(rotation);
        self.clear_parent_cache();
    }

    fn inner_get_rotation(&self) -> Rotation {
        self.rotation.borrow().clone()
    }

    fn inner_is_additive_blend(&self) -> bool {
        self.additive_blend.borrow().clone()
    }

    fn inner_set_additive_blend(&self, additive_blend: bool) {
        self.additive_blend.replace(additive_blend);
    }

    fn inner_is_mouse_hover(&self) -> bool {
        let p = get_mouse_position();
        let ap = self.inner_get_absolute_render_rect();
        match self.inner_get_conflict_type() {
            ConflictType::Square => {
                ap.contains_point(*p)
            },
            ConflictType::Circle => {
                Circle::from(ap).contains_point(p)
            }
        }
    }

    fn inner_set_conflict_type(&self, conflict_type: ConflictType) {
        self.conflict_type.replace(conflict_type);
    }

    fn inner_get_conflict_type(&self) -> ConflictType {
        self.conflict_type.borrow().clone()
    }

    fn inner_is_conflict(&self, other: Rc<dyn NodeLike>) -> bool {
        let ctype = self.inner_get_conflict_type();
        let other_ctype = self.inner_get_conflict_type();
        let (r1, r2) = (self.inner_get_absolute_render_rect(), other.inner_get_absolute_render_rect());
        if ctype == ConflictType::Square && other_ctype == ConflictType::Square {
            r1.has_intersection(*r2)
        } else if ctype == ConflictType::Circle && other_ctype == ConflictType::Square {
            Circle::from(r1).has_rect_intersection(&r2)
        } else if ctype == ConflictType::Square && other_ctype == ConflictType::Circle {
            Circle::from(r2).has_rect_intersection(&r1)
        } else {
            Circle::from(r1).has_intersection(&Circle::from(r2))
        }
    }

    fn inner_run_action(&self, action: Rc<dyn ActionLike>, option: RunActionOption) {
        self.next_actions.borrow_mut().push(action.clone());
        if let Some(n) = option.name.clone() {
            self.action_map.borrow_mut().insert(n, action);
        }
    }

    fn inner_get_action(&self, name: String) -> Option<Rc<dyn ActionLike>> {
        self.action_map.borrow().get(&name).cloned()
    }

    fn inner_destroy(&self) {
        self.clear_cache();
        let id = self.id();
        if let Some(parent) = self.inner_get_parent() {
            parent.inner_remove_child(id.clone());
        }
        for child in self.inner_get_children() {
            child.inner_destroy();
        }
        director::destroy_node(&id);
    }

}

impl <T> Node<T> where T: NodeDelegate + Any {

    pub fn create(delegate: T) -> Rc<Self> {
        let s = Rc::new(Self::new(delegate));
        director::register_node(s.clone());
        s
    }

    fn new(delegate: T) -> Self {
        Self {
            delegate: delegate,
            additive_blend: RefCell::new(false),
            absolute_position: RefCell::new(Point::new(0, 0)),
            position: RefCell::new(Point::new(0, 0)),
            anchor_point: RefCell::new(AnchorPoint::default()),
            parent: RefCell::new(None),
            opacity: RefCell::new(Opacity::from(1.0)),
            rotation: RefCell::new(Rotation::from(0.0)),
            visible: RefCell::new(true),
            scale: RefCell::new(Scale::from(1.0)),
            render_cache: RefCell::new(None),
            children: RefCell::new(Vec::new()),
            child_map: RefCell::new(HashMap::new()),
            conflict_type: RefCell::new(ConflictType::Square),
            actions: RefCell::new(Vec::new()),
            action_map: RefCell::new(HashMap::new()),
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
        if let Some(parent) = self.inner_get_parent() {
            parent.inner_clear_cache();
        }
    }

    fn generate_render_rect(&self, position: &Point) -> Rect {
        let ap = self.get_anchor_point();
        let s = self.inner_get_scaled_size();
        Rect::new(
            position.x() - ((s.width() as f32 * ap.x()).round() as i32),
            position.y() - ((s.height() as f32 * ap.y()).round() as i32),
            s.width(),
            s.height()
        )
    }

    fn remove_finished_actions(&self) {
        let mut next_actions: Vec<Rc<dyn ActionLike>> = Vec::new();
        let mut next_action_map: HashMap<String, Rc<dyn ActionLike>> = HashMap::new();
        for action in self.actions.borrow().iter() {
            if action.get_status() != ActionStatus::Finish {
                next_actions.push(action.clone());
            }
        }
        for (key, action) in self.action_map.borrow().iter() {
            if action.get_status() != ActionStatus::Finish {
                next_action_map.insert(key.to_string(), action.clone());
            }
        }
        self.actions.replace(next_actions);
        self.action_map.replace(next_action_map);
    }

    fn restore_next_action(&self) {
        let mut actions = self.actions.borrow_mut();
        let mut next_actions = self.next_actions.borrow_mut();
        actions.append(&mut next_actions);
    }

}

