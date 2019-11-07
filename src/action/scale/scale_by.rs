use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };

pub struct ScaleBy {
    to: f64,
    from: RefCell<Option<f64>>
}

impl ScaleBy {

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

impl ActionDelegate for ScaleBy {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.get_scale()));
        }
        let from = self.from.borrow().clone().unwrap();
        let ascale = from * (self.to - 1.0);
        node.set_scale(from + (ascale * progress as f64));
        None
    }

}
