use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };
use ::util::parameter::{ Point };

pub struct RelocateBy {
    to: Point,
    from: RefCell<Option<Point>>
}

impl RelocateBy {

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

impl ActionDelegate for RelocateBy {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.get_position()));
        }
        let from = self.from.borrow().clone().unwrap();
        node.set_position(&Point::new(
            from.x() + (self.to.x() as f32 * progress) as i32,
            from.y() + (self.to.y() as f32 * progress) as i32
        ));
        None
    }

}
