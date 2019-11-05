use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };

pub trait ActionDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus>;

}
