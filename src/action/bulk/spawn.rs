use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };
use ::util::easing::{ EasingFunction };

pub struct Spawn {
    actions: Vec<Rc<dyn ActionLike>>
}

impl Spawn {

    pub fn create(actions: Vec<Rc<dyn ActionLike>>) -> Rc<ParentAction<Spawn>> {
        ParentAction::create(|| {
            Self {
                actions: actions.clone(),
            }
        })
    }

}

impl ParentActionDelegate for Spawn {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        let statuses: Vec<ActionStatus> = self.actions.iter().map(|e| {
            e.run(node.clone(), easing.clone())
        }).collect();
        for status in &statuses {
            if status != &ActionStatus::Finish {
                return ActionStatus::Processing;
            }
        }
        ActionStatus::Finish
    }

    fn children(&self) -> Vec<Rc<dyn ActionLike>> {
        self.actions.clone()
    }

}
