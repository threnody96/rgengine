use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ NodeLike };
use ::action::{ ActionDelegate, ActionStatus, ActionLike };
use ::util::easing::{ EasingFunction };
use time::{ Tm };

pub struct Action<T> where T: ActionDelegate + Any {
    delegate: T,
    status: RefCell<ActionStatus>,
    prev_tm: RefCell<Option<Tm>>,
    progress: RefCell<f32>,
    duration: f64,
    speed: RefCell<f64>
}

impl <T> ActionLike for Action<T> where T: ActionDelegate + Any {

    fn run(&self, node: Rc<dyn NodeLike>, easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        if self.initialize() {
            let now = time::now();
            let time_progress = self.generate_progress(&now);
            let progress = easing.map(|e| e.ease(time_progress)).unwrap_or(time_progress);
            let status = self.delegate.run(node, progress);
            if time_progress == 1.0 || (status.is_some() && status.unwrap() == ActionStatus::Finish) {
                self.status.replace(ActionStatus::Finish);
            }
            self.prev_tm.replace(Some(now));
            self.progress.replace(time_progress);
        }
        self.get_status()
    }

    fn get_status(&self) -> ActionStatus {
        self.status.borrow().clone()
    }

    fn is_finish(&self) -> bool {
        self.get_status() == ActionStatus::Finish
    }

    fn set_speed(&self, speed: f64) {
        if speed <= 0.0 { panic!(format!("invalid action speed: {}", speed)); }
        self.speed.replace(speed);
    }

    fn get_speed(&self) -> f64 {
        self.speed.borrow().clone()
    }

    fn pause(&self) {
        match self.get_status() {
            ActionStatus::Wait | ActionStatus::Processing => {
                self.status.replace(ActionStatus::Pause);
            },
            ActionStatus::Finish | ActionStatus::Pause => {
            }
        }
    }

    fn resume(&self) {
        match self.get_status() {
            ActionStatus::Pause => {
                if self.prev_tm.borrow().is_none() {
                    self.status.replace(ActionStatus::Wait);
                } else {
                    self.prev_tm.replace(Some(time::now()));
                    self.status.replace(ActionStatus::Processing);
                }
            },
            _ => {
            }
        }
    }

}

impl <T> Action<T> where T: ActionDelegate + Any {

    pub fn create(duration: f64, delegate: T) -> Rc<Self> {
        Rc::new(Self::new(duration, delegate))
    }

    fn generate_progress(&self, now: &Tm) -> f32 {
        if self.duration == 0.0 { return 1.0; }
        if let Some(prev_tm) = self.prev_tm.borrow().clone() {
            let d = (*now - prev_tm).num_microseconds().unwrap();
            let dt = ((d as f64 / (self.duration * 1_000_000.0)) * self.get_speed()) as f32;
            let progress = self.progress.borrow().clone() + dt;
            if progress > 1.0 { 1.0 } else { progress }
        } else {
            0.0
        }
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

    fn new(duration: f64, delegate: T) -> Self {
        Self {
            delegate: delegate,
            status: RefCell::new(ActionStatus::Wait),
            prev_tm: RefCell::new(None),
            progress: RefCell::new(0.0),
            duration: duration,
            speed: RefCell::new(1.0)
        }
    }

}
