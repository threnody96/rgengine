use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };
use ::util::easing::{ EasingFunction };

pub struct Sequence {
    actions: Vec<Rc<dyn ActionLike>>,
}

impl Sequence {

    pub fn create(actions: Vec<Rc<dyn ActionLike>>) -> Rc<ParentAction<Sequence>> {
        ParentAction::create(Self {
            actions: actions.clone()
        })
    }

}

impl ParentActionDelegate for Sequence {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        for action in &self.actions {
            let status = action.run(node.clone(), easing.clone());
            if status != ActionStatus::Finish {
                return ActionStatus::Processing;
            }
        }
        ActionStatus::Finish
    }

    fn children(&self) -> Vec<Rc<dyn ActionLike>> {
        self.actions.clone()
    }

}
