use std::rc::Rc;
use std::cell::RefCell;
use ::util::{ director };
use ::util::parameter::{ Size, Point };
use ::node::{ NodeDelegate, Node, NodeLike };
use ::node::layer::{ LayerOption };

pub struct Layer {
    option: RefCell<LayerOption>
}

impl Layer {

    pub fn create<A>(option: A) -> Rc<Node<Layer>>
    where A: Into<LayerOption>
    {
        let o = option.into();
        let n = Node::create(|| Layer {
            option: RefCell::new(o.clone())
        });
        let size = n.get_size();
        n.set_position(&Point::new(size.width() as i32 / 2, size.height() as i32 / 2));
        n
    }

}

impl NodeDelegate for Layer {

    fn get_size(&self) -> Size {
        let option = self.option.borrow();
        if let Some(size) = option.size.clone() { return size; }
        director::get_resolution_size()
    }

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>) {
        let mut option = self.option.borrow_mut();
        if option.size.is_none() {
            option.size = Some(parent.get_size());
        }
    }

    fn update(&self) { }

    fn render(&self) { }

}

