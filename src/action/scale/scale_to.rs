use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };
use ::util::parameter::{ Scale };

pub struct ScaleTo {
    to: Scale,
    from: RefCell<Option<Scale>>
}

impl ScaleTo {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
        where A: Into<Scale>
    {
        let t = to.into();
        Action::create(duration, Self {
            to: t.clone(),
            from: RefCell::new(None)
        })
    }

}

impl ActionDelegate for ScaleTo {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.inner_get_scale()));
        }
        let from = self.from.borrow().clone().unwrap();
        let ascale = *self.to - *from;
        node.inner_set_scale(from + (ascale * progress as f64));
        None
    }

}
