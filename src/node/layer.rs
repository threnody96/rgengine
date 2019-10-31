use std::rc::Rc;
use ::util::{ director, Size, Point };
use ::node::{ NodeDelegate, Node, NodeLike };

pub struct Layer {
    size: Size
}

impl Layer {

    pub fn create() -> Rc<Node<Layer>> {
        let n = Node::create(|| Layer {
            size: director(|d| d.window_size())
        });
        let size = n.get_size();
        n.set_position(&Point::new(size.width() as i32 / 2, size.height() as i32 / 2));
        n
    }

}

impl NodeDelegate for Layer {

    fn get_size(&self) -> Size {
        self.size.clone()
    }

    fn update(&self, _parent: Rc<dyn NodeLike>) { }

    fn render(&self, _parent: Rc<dyn NodeLike>) { }

}

