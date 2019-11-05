use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct Linear {
    action: Rc<dyn ActionLike>
}

impl Linear {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<Linear>> {
        ParentAction::create(|| {
            Self {
                action: action.clone()
            }
        })
    }

}

impl ParentActionDelegate for Linear {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        self.action.run(node, Some(Box::new(|f| f)))
    }

}
