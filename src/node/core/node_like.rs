use std::rc::Rc;
use ::node::{ NodeId, AddChildOption };
use ::resource::{ RTexture, RFont };
use ::util::{ Point, AnchorPoint, Size };
use sdl2::pixels::{ Color };

pub trait NodeLike {

    fn id(&self) -> NodeId;

    fn get_size(&self) -> Size;

    fn get_render_point(&self) -> Point;

    fn update(&self);

    fn render(&self, parent: Option<Rc<dyn NodeLike>>);

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

    fn render_texture(&self, parent: &Option<Rc<dyn NodeLike>>, texture: Rc<RTexture>);

    fn render_label(&self, parent: &Option<Rc<dyn NodeLike>>, text: &str, font: Rc<RFont>, color: &Color);

    fn destroy(&self);

}

