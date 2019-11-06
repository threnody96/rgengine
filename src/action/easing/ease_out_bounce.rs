use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseOutBounce { }

impl EaseOutBounce {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.12, 0.11),
                (0.24, 0.44),
                (0.36, 0.98),
                (0.54, 0.75),
                (0.74, 0.98),
                (0.82, 0.94),
                (0.92, 0.99),
                (0.96, 0.98)
            )
        )
    }

}

