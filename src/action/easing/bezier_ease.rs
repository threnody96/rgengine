use std::rc::Rc;
use ::util::{ calc_bezier_curve };
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct BezierEase {
    action: Rc<dyn ActionLike>,
    points: Vec<(f32, f32)>
}

impl BezierEase {

    pub fn create(action: Rc<dyn ActionLike>, points: Vec<(f32, f32)>) -> Rc<ParentAction<BezierEase>> {
        ParentAction::create(|| {
            Self {
                action: action.clone(),
                points: points.clone()
            }
        })
    }

}

impl ParentActionDelegate for BezierEase {

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        let points = self.points.clone();
        self.action.run(node, &Some(Box::new(move |f| {
            calc_bezier_curve(&points, f).1
        })))
    }

}
