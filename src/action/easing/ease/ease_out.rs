use std::rc::Rc;
use ::action::{ ParentAction, ActionLike };
use ::action::easing::{ EasingAction };

pub struct EaseOut { }

impl EaseOut {

    pub fn create(action: Rc<dyn ActionLike>) -> Rc<ParentAction<EasingAction>> {
        EasingAction::create(action, ::util::easing::ease::EaseOut::create())
    }

}

