use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };

pub trait ParentActionDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus;

}
