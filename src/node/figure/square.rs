use std::cell::RefCell;
use std::rc::Rc;
use ::node::{ Node, NodeLike, NodeDelegate };
use ::util::parameter::{ Size };
use ::node::figure::{ SquareOption };

pub struct Square {
    size: RefCell<Size>,
    option: RefCell<SquareOption>
}

impl Square {

    pub fn create<A, B>(size: A, option: B) -> Rc<Node<Self>>
    where A: Into<Size>, B: Into<SquareOption>
    {
        let o = option.into();
        let n = Node::create(Self {
            size: RefCell::new(size.into()),
            option: RefCell::new(o.clone())
        });
        n
    }

}

impl NodeDelegate for Square {

    fn get_size(&self) -> Size {
        self.size.borrow().clone()
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self) { }

    fn render(&self) {
        self.render_square(&self.option.borrow().color);
    }

}
