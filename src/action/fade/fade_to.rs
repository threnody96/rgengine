use std::rc::Rc;
use std::cell::RefCell;
use ::action::{ Action, ActionDelegate, ActionStatus };
use ::node::{ NodeLike };
use ::util::parameter::{ Opacity };

pub struct FadeTo {
    to: Opacity,
    from: RefCell<Option<Opacity>>
}

impl FadeTo {

    pub fn create<A>(duration: f64, to: A) -> Rc<Action<Self>>
        where A: Into<Opacity>
    {
        let t = to.into();
        Action::create(duration, Self {
            to: t.clone(),
            from: RefCell::new(None)
        })
    }

}

impl ActionDelegate for FadeTo {

    fn run(&self, node: Rc<dyn NodeLike>, progress: f32) -> Option<ActionStatus> {
        if self.from.borrow().is_none() {
            self.from.replace(Some(node.inner_get_opacity()));
        }
        let from = self.from.borrow().clone().unwrap().opacity_rate();
        let ao = self.to.opacity_rate() - from;
        let o = from + (ao * progress as f64);
        node.inner_set_opacity(Opacity::from(o));
        None
    }

}
