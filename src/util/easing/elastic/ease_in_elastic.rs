use std::rc::Rc;
use ::util::easing::{ PolylineEase };

pub struct EaseInElastic { }

impl EaseInElastic {

    pub fn create() -> Rc<PolylineEase> {
        PolylineEase::create(
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

