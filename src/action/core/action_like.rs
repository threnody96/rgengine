use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };

pub trait ActionLike {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Box<Fn(f32) -> f32>>) -> ActionStatus;

    fn get_status(&self) -> ActionStatus;

}
