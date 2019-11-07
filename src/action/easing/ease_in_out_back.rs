use std::rc::Rc;
use ::action::easing::{ BezierEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInOutBack { }

impl EaseInOutBack {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<BezierEase>> {
        BezierEase::create(action, vec!((0.68, -0.55), (0.265, 1.55)))
    }

}

