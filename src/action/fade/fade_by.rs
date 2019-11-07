use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };

pub struct FadeBy {
    to: u8,
    from: RefCell<Option<u8>>
}

impl FadeBy {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
        where A: Into<u8>
    {
        let t = to.into();
        Action::create(duration, || {
            Self {
                to: t.clone(),
                from: RefCell::new(None)
            }
        })
    }

}

impl ActionDelegate for FadeBy {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.get_opacity()));
        }
        let from = self.from.borrow().clone().unwrap() as i64;
        let o = from + (self.to as f32 * progress) as i64;
        if o < 0 {
            node.set_opacity(0);
        } else if o > 255 {
            node.set_opacity(255);
        } else {
            node.set_opacity(o as u8);
        }
        None
    }

}
