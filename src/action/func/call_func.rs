use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };
use ::util::easing::{ EasingFunction };

pub struct CallFunc {
    callback: Rc<Fn(Rc<dyn NodeLike>) -> ()>
}

impl CallFunc {

    pub fn create(callback: Rc<Fn(Rc<dyn NodeLike>) -> ()>) -> Rc<ParentAction<CallFunc>> {
        ParentAction::create(|| {
            Self {
                callback: callback.clone()
            }
        })
    }

}

impl ParentActionDelegate for CallFunc {

    fn run(&self, node: Rc<dyn NodeLike>, _easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        (&self.callback)(node);
        ActionStatus::Finish
    }

    fn children(&self) -> Vec<Rc<dyn ActionLike>> {
        Vec::new()
    }

}
