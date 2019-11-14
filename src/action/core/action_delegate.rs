use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ActionStatus, ActionId };

pub trait ActionDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus>;

    fn id(&self) -> ActionId {
        ActionId::new(format!("{:p}", self))
    }

}
