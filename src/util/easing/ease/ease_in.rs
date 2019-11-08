use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseIn { }

impl EaseIn {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.42, 0.0), (1.0, 1.0)))
    }

}

