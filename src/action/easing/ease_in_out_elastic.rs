use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInOutElastic { }

impl EaseInOutElastic {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.08, 0.00),
                (0.18, -0.01),
                (0.20, 0.00),
                (0.28, 0.02),
                (0.30, 0.02),
                (0.38, -0.09),
                (0.40, -0.12),
                (0.60, 1.12),
                (0.62, 1.09),
                (0.70, 0.98),
                (0.72, 0.98),
                (0.80, 1.00),
                (0.82, 1.01),
                (0.90, 1.00),
                (0.92, 1.00)
            )
        )
    }

}

