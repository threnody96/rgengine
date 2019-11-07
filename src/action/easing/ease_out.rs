use std::rc::Rc;
use ::action::easing::{ BezierEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseOut { }

impl EaseOut {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<BezierEase>> {
        BezierEase::create(action, vec!((0.0, 0.0), (0.58, 1.0)))
    }

}

