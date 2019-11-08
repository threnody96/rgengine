use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseInOutSine { }

impl EaseInOutSine {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.445, 0.05), (0.55, 0.95)))
    }

}

