use std::thread::{ sleep };
use std::time::{ Duration };
use time::{ Tm };

pub struct FpsManager {
    fps: u32,
    dt: i64,
    max_retry: i64,
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

    pub fn run<P, U, R>(&mut self, prev_sleep_time: i64, prepare: P, update: U, render: R) -> i64
    where P: FnOnce() -> (), U: Fn() -> bool, R: FnOnce() -> () {
        let (total_time, is_sleep) = Self::measure(|| {
            let prev_over_time = if prev_sleep_time > 0 { prev_sleep_time } else { 0 };
            let (render_time, _) = Self::measure(|| {
                render();
                self.rendered();
            });
            let (prepare_time, _) = Self::measure(|| prepare());
            let mut update_time: i64 = 0;
            for i in 0..self.max_retry {
                loop {
                    let (utime, r) = Self::measure(|| update());
                    update_time += utime;
                    if r { break; }
                }
                let delay = (self.dt * (i + 1)) - (prev_over_time + prepare_time + render_time + update_time);
                if delay >= 0 {
                    if delay != 0 && i == 0 {
                        sleep(Duration::new(0, delay as u32));
                        return true;
                    }
                }
            }
            false
        });
        if is_sleep { total_time - self.dt } else { 0 }
    }

    fn generate_fps_param(fps: u32) -> (i64, i64) {
        (
            1_000_000_000 / (fps as i64),
            if fps / 10 == 0 { 1 } else { fps as i64 / 10 }
        )
    }

    fn measure<T, R>(callback: T) -> (i64, R) where T: FnOnce() -> R {
        let t1 = time::now();
        let r = callback();
        let t2 = time::now();
        (
            (t2 - t1).num_nanoseconds().unwrap(),
            r
        )
    }

    fn rendered(&mut self) {
        let now = time::now();
        while self.render_times.len() > 0 && (now - *self.render_times.first().unwrap()).num_seconds() > 9 {
            self.render_times.remove(0);
        }
        self.render_times.push(now);
    }

    pub fn fps(&self) -> usize {
        ((self.render_times.len() as f32) / 10.0).ceil() as usize
    }

}
