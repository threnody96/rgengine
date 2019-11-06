use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInElastic { }

impl EaseInElastic {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.18,  0.00),
                (0.26, -0.01),
                (0.28, -0.01),
                (0.40,  0.02),
                (0.42,  0.02),
                (0.56, -0.05),
                (0.58, -0.04),
                (0.72,  0.13),
                (0.86, -0.37)
            )
        )
    }

}

