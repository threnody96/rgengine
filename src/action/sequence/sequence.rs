use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct Sequence {
    actions: Vec<Rc<dyn ActionLike>>
}

impl Sequence {

    pub fn create(actions: Vec<Rc<dyn ActionLike>>) -> Rc<ParentAction<Sequence>> {
        ParentAction::create(|| {
            Self {
                actions: actions.clone(),
            }
        })
    }

}

impl ParentActionDelegate for Sequence {

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        for action in &self.actions {
            let status = action.run(node.clone(), easing);
            if status != ActionStatus::End {
                return ActionStatus::Processing;
            }
        }
        ActionStatus::End
    }

}
