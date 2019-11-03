use std::rc::Rc;
use ::node::{ NodeId, AddChildOption };
use ::resource::{ Texture, Font, ResourceKey };
use ::util::parameter::{ Point, AnchorPoint, Size };
use sdl2::pixels::{ Color };

pub trait NodeLike {

    fn id(&self) -> NodeId;

    fn node(&self) -> Rc<dyn NodeLike>;

    fn get_size(&self) -> Size;

    fn get_render_point(&self) -> Point;

    fn update(&self, parent: Rc<dyn NodeLike>);

    fn update_children(&self, parent: Rc<dyn NodeLike>);

    fn render(&self, parent: Rc<dyn NodeLike>);

    fn render_children(&self, parent: Rc<dyn NodeLike>);

    fn use_cache(&self) -> bool;

    fn get_cache(&self) -> Option<ResourceKey>;

    fn set_cache(&self, key: Option<ResourceKey>);

    fn clear_cache(&self);

    fn add_parent(&self, id: &NodeId);

    fn remove_parent(&self, id: &NodeId);

    fn get_parents(&self) -> Vec<NodeId>;

    fn get_children(&self) -> Vec<NodeId>;

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption);

    fn remove_child(&self, id: &NodeId);

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

    fn set_position(&self, point: &Point);

    fn get_position(&self) -> Point;

    fn set_anchor_point(&self, anchor_point: &AnchorPoint);

    fn get_anchor_point(&self) -> AnchorPoint;

    fn set_opacity(&self, opacity: u8);

    fn get_opacity(&self) -> u8;

    fn set_visible(&self, visible: bool);

    fn get_visible(&self) -> bool;

    fn set_rotation(&self, rotation: f64);

    fn get_rotation(&self) -> f64;

    fn render_texture(&self, texture: Rc<Texture>);

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color);

    fn destroy(&self);

}

