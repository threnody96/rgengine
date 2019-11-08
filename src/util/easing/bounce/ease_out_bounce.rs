use std::rc::Rc;
use ::util::easing::{ PolylineEase };

pub struct EaseOutBounce { }

impl EaseOutBounce {

    pub fn create() -> Rc<PolylineEase> {
        PolylineEase::create(
            vec!(
                (0.12, 0.11),
                (0.24, 0.44),
                (0.36, 0.98),
                (0.54, 0.75),
                (0.74, 0.98),
                (0.82, 0.94),
                (0.92, 0.99),
                (0.96, 0.98)
            )
        )
    }

}

