use std::rc::Rc;
use ::node::{ NodeId, AddChildOption, ConflictType };
use ::resource::{ Texture, Font, ResourceKey };
use ::action::{ ActionLike };
use ::util::parameter::{ Point, AnchorPoint, Size, Rect, Color };

pub trait NodeLike {

    fn id(&self) -> NodeId;

    fn node(&self) -> Rc<dyn NodeLike>;

    fn get_size(&self) -> Size;

    fn get_scaled_size(&self) -> Size;

    fn get_render_rect(&self) -> Rect;

    fn get_absolute_render_rect(&self) -> Rect;

    fn update(&self);

    fn update_children(&self);

    fn render(&self);

    fn render_children(&self);

    fn use_cache(&self) -> bool;

    fn get_cache(&self) -> Option<ResourceKey>;

    fn set_cache(&self, key: Option<ResourceKey>);

    fn clear_cache(&self);

    fn set_parent(&self, id: &NodeId);

    fn remove_parent(&self);

    fn get_parent(&self) -> Option<Rc<dyn NodeLike>>;

    fn get_children(&self) -> Vec<Rc<dyn NodeLike>>;

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption);

    fn remove_child(&self, id: &NodeId);

    fn before_add_child(&self, child: Rc<dyn NodeLike>);

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>);

    fn set_position(&self, point: &Point);

    fn get_position(&self) -> Point;

    fn set_scale(&self, scale: f64);

    fn get_scale(&self) -> f64;

    fn update_absolute_position(&self);

    fn get_absolute_position(&self) -> Point;

    fn set_anchor_point(&self, anchor_point: &AnchorPoint);

    fn get_anchor_point(&self) -> AnchorPoint;

    fn set_opacity(&self, opacity: u8);

    fn get_opacity(&self) -> u8;

    fn set_visible(&self, visible: bool);

    fn get_visible(&self) -> bool;

    fn set_rotation(&self, rotation: f64);

    fn get_rotation(&self) -> f64;

    fn is_additive_blend(&self) -> bool;

    fn set_additive_blend(&self, additive_blend: bool);

    fn render_texture(&self, texture: Rc<Texture>);

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color);

    fn is_mouse_hover(&self) -> bool;

    fn set_conflict_type(&self, conflict_type: ConflictType);

    fn get_conflict_type(&self) -> ConflictType;

    fn is_conflict(&self, other: Rc<dyn NodeLike>) -> bool;

    fn run_action(&self, action: Rc<dyn ActionLike>);

    fn destroy(&self);

}

