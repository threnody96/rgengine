use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };

pub trait ParentActionDelegate {

    fn run<C>(&self, node: Rc<dyn NodeLike>, easing: C) -> ActionStatus where C: Fn(f32) -> f32;

}
