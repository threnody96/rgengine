use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };
use ::util::easing::{ EasingFunction };

pub struct EasingAction {
    action: Rc<dyn ActionLike>,
    easing: Rc<dyn EasingFunction>
}

impl EasingAction {

    pub fn create(action: Rc<dyn ActionLike>, easing: Rc<dyn EasingFunction>) -> Rc<ParentAction<EasingAction>> {
        ParentAction::create(|| {
            Self {
                action: action.clone(),
                easing: easing.clone()
            }
        })
    }

}

impl ParentActionDelegate for EasingAction {

    fn run(&self, node: Rc<dyn NodeLike>, _easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        self.action.run(node, Some(self.easing.clone()))
    }

}
