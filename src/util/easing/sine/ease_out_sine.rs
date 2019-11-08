use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseOutSine { }

impl EaseOutSine {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.39, 0.575), (0.565, 1.0)))
    }

}

