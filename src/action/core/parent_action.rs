use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionStatus, ActionLike };
use ::util::easing::{ EasingFunction };

pub struct ParentAction<T> where T: ParentActionDelegate + Any {
    delegate: T,
    status: RefCell<ActionStatus>,
    speed: RefCell<f64>
}

impl <T> ActionLike for ParentAction<T> where T: ParentActionDelegate + Any {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        if self.initialize() {
            let status = self.delegate.run(node, easing);
            if status == ActionStatus::Finish {
                self.status.replace(ActionStatus::Finish);
            }
        }
        self.get_status()
    }

    fn get_status(&self) -> ActionStatus {
        self.status.borrow().clone()
    }

    fn set_speed(&self, speed: f64) {
        self.speed.replace(speed);
        self.delegate.set_speed(speed);
    }

    fn get_speed(&self) -> f64 {
        self.speed.borrow().clone()
    }

    fn pause(&self) {
        self.status.replace(ActionStatus::Pause);
        self.delegate.pause_children();
    }

    fn resume(&self) {
        self.status.replace(ActionStatus::Processing);
        self.delegate.resume_children();
    }

}

impl <T> ParentAction<T> where T: ParentActionDelegate + Any {

    pub fn create<C>(callback: C) -> Rc<Self> where C: Fn() -> T {
        Rc::new(Self::new(callback()))
    }

    fn initialize(&self) -> bool {
        match self.get_status() {
            ActionStatus::Wait => {
                self.status.replace(ActionStatus::Processing);
                true
            },
            ActionStatus::Processing => {
                true
            },
            ActionStatus::Finish => {
                false
            },
            ActionStatus::Pause => {
                false
            }
        }
    }

    fn new(delegate: T) -> Self {
        Self {
            delegate: delegate,
            status: RefCell::new(ActionStatus::Wait),
            speed: RefCell::new(1.0)
        }
    }

}
