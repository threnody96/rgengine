use std::rc::Rc;
use ::node::{ NodeDelegate, Node, NodeLike };

pub struct Layer { }

impl Layer {

    pub fn create() -> Rc<Node<Layer>> {
        Node::create(|| Layer {})
    }

}

impl NodeDelegate for Layer {

    fn update(&self) { }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
    }

}

