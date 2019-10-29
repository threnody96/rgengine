use std::thread::{ sleep };
use std::time::{ Duration };
use time::{ Tm };

pub struct FpsManager {
    fps: u32,
    dt: f64,
    max_retry: u32,
    render_times: Vec<Tm>
}

impl FpsManager {

    pub fn new(fps: u32) -> Self {
        let fps_param = Self::generate_fps_param(fps);
        Self {
            render_times: Vec::new(),
            dt: fps_param.0,
            max_retry: fps_param.1,
            fps: fps
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        let fps_param = Self::generate_fps_param(fps);
        self.dt = fps_param.0;
        self.max_retry = fps_param.1;
        self.fps = fps;
    }

    pub fn run<P, U, R>(&mut self, prepare: P, update: U, render: R)
    where P: FnOnce() -> (), U: Fn() -> bool, R: FnOnce() -> () {
        let (render_time, _) = Self::measure(|| {
            render();
            self.rendered();
        });
        let (prepare_time, _) = Self::measure(|| prepare());
        let mut update_time: u64 = 0;
        for i in 0..self.max_retry {
            loop {
                let (utime, r) = Self::measure(|| update());
                update_time += utime;
                if r { break; }
            }
            let delay = (self.dt * (i + 1) as f64) - (prepare_time + render_time + update_time) as f64;
            if delay >= 0.0 {
                if delay != 0.0 && i == 0 {
                    sleep(Duration::new(0, delay as u32 * 1_000));
                }
                break;
            }
        }
    }

    fn generate_fps_param(fps: u32) -> (f64, u32) {
        (
            1.0 / (fps as f64) * 1_000_000.0,
            if fps / 10 == 0 { 1 } else { fps / 10 }
        )
    }

    fn measure<T, R>(callback: T) -> (u64, R) where T: FnOnce() -> R {
        let t1 = time::now();
        let r = callback();
        let t2 = time::now();
        (
            (t2 - t1).num_microseconds().unwrap() as u64,
            r
        )
    }

    fn rendered(&mut self) {
        let now = time::now();
        while self.render_times.len() > 0 && (now - *self.render_times.first().unwrap()).num_seconds() > 0 {
            self.render_times.remove(0);
        }
        self.render_times.push(now);
    }

    pub fn fps(&self) -> usize {
        self.render_times.len()
    }

}
