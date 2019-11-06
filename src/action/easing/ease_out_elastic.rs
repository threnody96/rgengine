use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseOutElastic { }

impl EaseOutElastic {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.16, 1.32),
                (0.28, 0.87),
                (0.44, 1.05),
                (0.59, 0.98),
                (0.73, 1.01),
                (0.88, 1.00)
            )
        )
    }

}

