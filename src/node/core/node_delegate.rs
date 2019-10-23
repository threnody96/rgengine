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

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        director(|d| {
            let n = d.get_nodelike(self.id()).unwrap();
            n.add_child(node, option);
        })
    }

}

