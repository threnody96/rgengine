use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus, ActionLike };
use ::util::easing::{ EasingFunction };

pub trait ParentActionDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus;

    fn children(&self) -> Vec<Rc<dyn ActionLike>>;

    fn set_speed(&self, speed: f64) {
        for child in &self.children() {
            child.set_speed(speed);
        }
    }

    fn pause_children(&self) {
        for child in &self.children() {
            child.pause();
        }
    }

    fn resume_children(&self) {
        for child in &self.children() {
            child.resume();
        }
    }

}
