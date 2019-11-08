use std::rc::Rc;
use ::util::easing::{ BezierEase };

pub struct EaseInSine { }

impl EaseInSine {

    pub fn create() -> Rc<BezierEase> {
        BezierEase::create(vec!((0.47, 0.0), (0.745, 0.715)))
    }

}

