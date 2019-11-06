use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInOutBounce { }

impl EaseInOutBounce {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.02, 0.01),
                (0.04, 0.00),
                (0.10, 0.03),
                (0.14, 0.01),
                (0.22, 0.12),
                (0.32, 0.01),
                (0.42, 0.40),
                (0.50, 0.50),
                (0.58, 0.60),
                (0.68, 0.99),
                (0.78, 0.88),
                (0.86, 0.99),
                (0.90, 0.97),
                (0.96, 1.00),
                (0.98, 0.99)
            )
        )
    }

}

