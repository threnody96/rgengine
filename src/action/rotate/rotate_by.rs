use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };

pub struct RotateBy {
    to: f64,
    from: RefCell<Option<f64>>
}

impl RotateBy {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
        where A: Into<f64>
    {
        let t = to.into();
        Action::create(duration, || {
            Self {
                to: t.clone(),
                from: RefCell::new(None)
            }
        })
    }

}

impl ActionDelegate for RotateBy {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.get_rotation()));
        }
        let from = self.from.borrow().clone().unwrap();
        node.set_rotation(from + (self.to * progress as f64));
        None
    }

}
