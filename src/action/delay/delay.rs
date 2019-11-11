use std::rc::Rc;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };

pub struct Delay { }

impl Delay {

    pub fn create(duration: f64) -> Rc<Action<Self>> {
        Action::create(duration, Self { })
    }

}

impl ActionDelegate for Delay {

    fn run(&self, _node: Rc<dyn NodeLike>, _progress: f32) -> Option<ActionStatus> {
        None
    }

}
