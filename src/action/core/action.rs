use std::time::{ Duration };
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ NodeLike };
use ::action::{ ActionDelegate, ActionStatus, ActionLike };
use time::{ Tm };

pub struct Action<T> where T: ActionDelegate + Any {
    delegate: T,
    status: RefCell<ActionStatus>,
    start: RefCell<Option<Tm>>,
    duration: f64
}

impl <T> ActionLike for Action<T> where T: ActionDelegate + Any {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        if let Some(now) = self.initialize() {
            let time_progress = self.generate_progress(now);
            let progress = easing.map(|e| e(time_progress)).unwrap_or(time_progress);
            let status = self.delegate.run(node, progress);
            if time_progress == 1.0 || (status.is_some() && status.unwrap() == ActionStatus::End) {
                self.status.replace(ActionStatus::End);
            }
        }
        self.get_status()
    }

    fn get_status(&self) -> ActionStatus {
        self.status.borrow().clone()
    }

}

impl <T> Action<T> where T: ActionDelegate + Any {

    pub fn create<C>(duration: f64, callback: C) -> Rc<Self> where C: Fn() -> T {
        Rc::new(Self::new(duration, callback()))
    }

    fn generate_progress(&self, now: Tm) -> f32 {
        let start = self.start.borrow().clone().unwrap();
        let d = (now - start).num_microseconds().unwrap();
        if self.duration == 0.0 { return 1.0; }
        let progress = d as f64 / (self.duration * 1_000_000.0);
        if progress > 1.0 { 1.0 } else { progress as f32 }
    }

    fn initialize(&self) -> Option<Tm> {
        match self.get_status() {
            ActionStatus::Wait => {
                let now = time::now();
                self.status.replace(ActionStatus::Processing);
                self.start.replace(Some(now.clone()));
                Some(now)
            },
            ActionStatus::Processing => {
                Some(time::now())
            },
            ActionStatus::End => {
                None
            }
        }
    }

    fn new(duration: f64, delegate: T) -> Self {
        Self {
            delegate: delegate,
            status: RefCell::new(ActionStatus::Wait),
            start: RefCell::new(None),
            duration: duration
        }
    }

}
