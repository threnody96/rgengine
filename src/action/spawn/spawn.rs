use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

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

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        let statuses: Vec<ActionStatus> = self.actions.iter().map(|e| {
            e.run(node.clone(), easing)
        }).collect();
        for status in &statuses {
            if status != &ActionStatus::End {
                return ActionStatus::Processing;
            }
        }
        ActionStatus::End
    }

}
