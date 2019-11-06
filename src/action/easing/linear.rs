use std::rc::Rc;
use ::action::easing::{ PolylineEase };
use ::action::{ ParentAction, ActionLike };

pub struct Linear { }

impl Linear {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<PolylineEase>> {
        PolylineEase::create(action, vec!())
    }

}

