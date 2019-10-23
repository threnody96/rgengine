use std::rc::Rc;
use ::node::{ NodeId, AddChildOption };
use ggez::{ Context };

pub trait NodeLike {

    fn id(&self) -> NodeId;

    fn update(&self);

    fn render(&self, ctx: &mut Context);

    fn add_parent(&self, id: NodeId);

    fn remove_parent(&self, id: NodeId);

    fn get_parents(&self) -> Vec<NodeId>;

    fn get_children(&self) -> Vec<NodeId>;

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption);

    fn remove_child(&self, id: NodeId);

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

}

