use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseInBack { }

impl EaseInBack {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.6, -0.28), (0.735, 0.045)))
    }

}

