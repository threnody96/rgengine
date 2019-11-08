use std::rc::Rc;
use ::util::easing::{ PolylineEase };

pub struct EaseInBounce { }

impl EaseInBounce {

    pub fn create() -> Rc<PolylineEase> {
        PolylineEase::create(
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

