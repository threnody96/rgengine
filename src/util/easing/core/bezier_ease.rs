use std::rc::Rc;
use ::util::easing::{ EasingFunction };

pub struct BezierEase {
    points: Vec<(f32, f32)>
}

impl BezierEase {

    pub fn create(mut points: Vec<(f32, f32)>) -> Rc<BezierEase> {
        let mut p: Vec<(f32, f32)> = vec!((0.0, 0.0));
        p.append(&mut points);
        p.push((1.0, 1.0));
        Rc::new(Self {
            points: p.clone()
        })
    }

    fn factorial(n: i32) -> i32 {
        if n <= 1 { return 1; }
        let mut result: i32 = 1;
        for i in 2i32..(n + 1) {
            result *= i
        }
        result
    }

}

impl EasingFunction for BezierEase {

    fn ease(&self, t: f32) -> f32 {
        let mut result: (f32, f32) = (0.0, 0.0);
        for i in 0i32..(self.points.len() as i32) {
            let n = self.points.len() as i32 - 1;
            let f = Self::factorial(n) / (Self::factorial(i) * Self::factorial(n - i));
            let j = (f as f32) * t.powi(i) * (1.0 - t).powi(n - i);
            let p = self.points.get(i as usize).unwrap();
            result.0 += p.0 * j;
            result.1 += p.1 * j;
        }
        result.1
    }

}
