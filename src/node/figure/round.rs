use std::cell::RefCell;
use std::rc::Rc;
use ::node::{ Node, ConflictType, NodeLike, NodeDelegate };
use ::util::parameter::{ Size };
use ::node::figure::{ RoundOption };

pub struct Round {
    diameter: RefCell<u32>,
    option: RefCell<RoundOption>
}

impl Round {

    pub fn create<A>(diameter: u32, option: A) -> Rc<Node<Self>>
    where A: Into<RoundOption>
    {
        let o = option.into();
        let n = Node::create(|| {
            Self {
                diameter: RefCell::new(diameter),
                option: RefCell::new(o.clone())
            }
        });
        n.set_conflict_type(ConflictType::Circle);
        n
    }

}

impl NodeDelegate for Round {

    fn get_size(&self) -> Size {
        let d = self.diameter.borrow().clone();
        Size::new(d, d)
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self) { }

    fn render(&self) {
        self.render_round(&self.option.borrow().color);
    }


}