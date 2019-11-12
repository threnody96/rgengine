use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };
use ::util::parameter::{ Point };

pub struct RelocateTo {
    to: Point,
    from: RefCell<Option<Point>>
}

impl RelocateTo {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
    where A: Into<Point>
    {
        let t = to.into();
        Action::create(duration, Self {
            to: t.clone(),
            from: RefCell::new(None)
        })
    }

}

impl ActionDelegate for RelocateTo {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.inner_get_position()));
        }
        let from = self.from.borrow().clone().unwrap();
        let (ax, ay) = (self.to.x() - from.x(), self.to.y() - from.y());
        node.inner_set_position(Point::new(
            from.x() + (ax as f32 * progress) as i32,
            from.y() + (ay as f32 * progress) as i32
        ));
        None
    }

}
