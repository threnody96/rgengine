use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };
use ::util::easing::{ EasingFunction };

pub trait ParentActionDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus;

}
