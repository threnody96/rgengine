use std::rc::Rc;
use ::node::{ NodeId, NodeLike, AddChildOption };
use ::util::{ director };

pub trait NodeDelegate {

    fn update(&self);

    fn render(&self);

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

    fn id(&self) -> NodeId {
        NodeId::new(format!("{:p}", self))
    }

    fn node(&self) -> Rc<dyn NodeLike> {
        director(|d| {
            let id = self.id();
            d.get_nodelike(&id).unwrap()
        })
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.node().add_child(node, option);
    }

    fn get_children(&self) -> Vec<NodeId> {
        self.node().get_children()
    }

}

