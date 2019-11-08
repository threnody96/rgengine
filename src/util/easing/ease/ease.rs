use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct Ease { }

impl Ease {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.25, 0.1), (0.25, 1.0)))
    }

}

