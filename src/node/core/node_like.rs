use std::rc::Rc;
use ::node::{ NodeId, AddChildOption, ConflictType };
use ::resource::{ Texture, Font, ResourceKey };
use ::action::{ ActionLike };
use ::util::parameter::{ Point, AnchorPoint, Size, Rect, Color, Opacity, Scale, Rotation };

pub trait NodeLike {

    fn inner_id(&self) -> NodeId;

    fn inner_node(&self) -> Rc<dyn NodeLike>;

    fn inner_get_size(&self) -> Size;

    fn inner_get_scaled_size(&self) -> Size;

    fn inner_get_render_rect(&self) -> Rect;

    fn inner_get_absolute_render_rect(&self) -> Rect;

    fn inner_update(&self);

    fn inner_update_children(&self);

    fn inner_render(&self);

    fn inner_render_children(&self);

    fn inner_use_cache(&self) -> bool;

    fn inner_get_cache(&self) -> Option<ResourceKey>;

    fn inner_set_cache(&self, key: Option<ResourceKey>);

    fn inner_clear_cache(&self);

    fn inner_set_parent(&self, id: NodeId);

    fn inner_remove_parent(&self);

    fn inner_get_parent(&self) -> Option<Rc<dyn NodeLike>>;

    fn inner_get_children(&self) -> Vec<Rc<dyn NodeLike>>;

    fn inner_before_add_child(&self, child: Rc<dyn NodeLike>);

    fn inner_before_be_added_child(&self, parent: Rc<dyn NodeLike>);

    fn inner_add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption);

    fn inner_remove_child(&self, id: NodeId);

    fn inner_set_position(&self, point: Point);

    fn inner_get_position(&self) -> Point;

    fn inner_set_scale(&self, scale: Scale);

    fn inner_get_scale(&self) -> Scale;

    fn inner_update_absolute_position(&self);

    fn inner_get_absolute_position(&self) -> Point;

    fn inner_set_anchor_point(&self, anchor_point: AnchorPoint);

    fn inner_get_anchor_point(&self) -> AnchorPoint;

    fn inner_set_opacity(&self, opacity: Opacity);

    fn inner_get_opacity(&self) -> Opacity;

    fn inner_set_visible(&self, visible: bool);

    fn inner_get_visible(&self) -> bool;

    fn inner_set_rotation(&self, rotation: Rotation);

    fn inner_get_rotation(&self) -> Rotation;

    fn inner_is_additive_blend(&self) -> bool;

    fn inner_set_additive_blend(&self, additive_blend: bool);

    fn inner_prepare_render_tree(&self);

    fn inner_render_texture(&self, texture: Rc<Texture>);

    fn inner_render_label(&self, text: &str, font: Rc<Font>, color: &Color);

    fn inner_is_mouse_hover(&self) -> bool;

    fn inner_set_conflict_type(&self, conflict_type: ConflictType);

    fn inner_get_conflict_type(&self) -> ConflictType;

    fn inner_is_conflict(&self, other: Rc<dyn NodeLike>) -> bool;

    fn inner_run_action(&self, action: Rc<dyn ActionLike>);

    fn inner_destroy(&self);

}

