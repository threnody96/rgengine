use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };
use ::util::parameter::{ Rotation };

pub struct RotateBy {
    to: Rotation,
    from: RefCell<Option<Rotation>>
}

impl RotateBy {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
        where A: Into<Rotation>
    {
        let t = to.into();
        Action::create(duration, Self {
            to: t.clone(),
            from: RefCell::new(None)
        })
    }

}

impl ActionDelegate for RotateBy {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.inner_get_rotation()));
        }
        let from = self.from.borrow().clone().unwrap();
        node.inner_set_rotation(from + (self.to * progress as f64));
        None
    }

}
