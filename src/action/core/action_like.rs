use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus };
use ::util::easing::{ EasingFunction };

pub trait ActionLike {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus;

    fn get_status(&self) -> ActionStatus;

    fn set_speed(&self, speed: f64);

    fn get_speed(&self) -> f64;

    fn pause(&self);

    fn resume(&self);

}
