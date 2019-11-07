use std::rc::Rc;
use ::action::easing::{ BezierEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInSine { }

impl EaseInSine {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<BezierEase>> {
        BezierEase::create(action, vec!((0.47, 0.0), (0.745, 0.715)))
    }

}

