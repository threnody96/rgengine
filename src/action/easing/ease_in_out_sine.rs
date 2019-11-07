use std::rc::Rc;
use ::action::easing::{ BezierEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInOutSine { }

impl EaseInOutSine {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<BezierEase>> {
        BezierEase::create(action, vec!((0.445, 0.05), (0.55, 0.95)))
    }

}

