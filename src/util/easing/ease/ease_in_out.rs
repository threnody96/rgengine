use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseInOut { }

impl EaseInOut {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.42, 0.0), (0.58, 1.0)))
    }

}

