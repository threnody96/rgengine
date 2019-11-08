use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseOutBack { }

impl EaseOutBack {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.175, 0.885), (0.32, 1.275)))
    }

}

