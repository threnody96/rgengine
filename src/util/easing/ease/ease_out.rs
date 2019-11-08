use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseOut { }

impl EaseOut {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.0, 0.0), (0.58, 1.0)))
    }

}

