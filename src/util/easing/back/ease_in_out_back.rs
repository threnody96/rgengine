use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseInOutBack { }

impl EaseInOutBack {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.68, -0.55), (0.265, 1.55)))
    }

}

