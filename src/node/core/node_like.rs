use std::rc::Rc;
use ::node::{ NodeId, AddChildOption };
use ::resource::{ RTexture, RFont };
use ::util::{ Point };
use sdl2::pixels::{ Color };

pub trait NodeLike {

    fn id(&self) -> NodeId;

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

    fn render_texture(&self, parent: &Option<Rc<dyn NodeLike>>, texture: &RTexture);

    fn render_label(&self, parent: &Option<Rc<dyn NodeLike>>, text: &str, font: &RFont, color: &Color);

}

