use std::rc::Rc;
use ::action::easing::{ BezierEase };
use ::action::{ ParentAction, ActionLike };

pub struct Ease { }

impl Ease {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<BezierEase>> {
        BezierEase::create(action, vec!((0.0, 0.0), (0.25, 0.1), (0.25, 1.0), (1.0, 1.0)))
    }

}

