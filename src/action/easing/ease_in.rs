use std::rc::Rc;
use ::util::{ calc_bezier_curve };
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct EaseIn {
    action: Rc<dyn ActionLike>
}

impl EaseIn {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<EaseIn>> {
        ParentAction::create(|| {
            Self {
                action: action.clone()
            }
        })
    }

}

impl ParentActionDelegate for EaseIn {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        self.action.run(node, Some(Box::new(|f| {
            let (_, y) = calc_bezier_curve(vec!((0.0, 0.0), (0.42, 0.0), (1.0, 1.0), (1.0, 1.0)), f);
            y
        })))
    }

}
