use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct EaseInBounce { }

impl EaseInBounce {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(
            action,
            vec!(
                (0.04, 0.02),
                (0.08, 0.01),
                (0.18, 0.06),
                (0.26, 0.02),
                (0.46, 0.25),
                (0.64, 0.02),
                (0.76, 0.56),
                (0.88, 0.89)
            )
        )
    }

}

