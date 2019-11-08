use std::rc::Rc;
use ::util::easing::{ PolylineEase };

pub struct Linear { }

impl Linear {

    pub fn create() -> Rc<PolylineEase> {
        PolylineEase::create(vec!())
    }

}

