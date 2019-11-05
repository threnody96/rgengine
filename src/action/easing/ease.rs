use std::rc::Rc;
use ::util::{ calc_bezier_curve };
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct Ease {
    action: Rc<dyn ActionLike>
}

impl Ease {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<Ease>> {
        ParentAction::create(|| {
            Self {
                action: action.clone()
            }
        })
    }

}

impl ParentActionDelegate for Ease {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        self.action.run(node, Some(Box::new(|f| {
            let (_, y) = calc_bezier_curve(vec!((0.0, 0.0), (0.25, 0.1), (0.25, 1.0), (1.0, 1.0)), f);
            y
        })))
    }

}
