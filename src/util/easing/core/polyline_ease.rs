use std::rc::Rc;
use ::util::easing::{ EasingFunction };

pub struct PolylineEase {
    points: Vec<(f32, f32)>
}

impl PolylineEase {

    pub fn create(mut points: Vec<(f32, f32)>) -> Rc<PolylineEase> {
        let mut p: Vec<(f32, f32)> = vec!((0.0, 0.0));
        p.append(&mut points);
        p.push((1.0, 1.0));
        Rc::new(Self {
            points: p.clone()
        })
    }

}

impl EasingFunction for PolylineEase {

    fn ease(&self, t: f32) -> f32 {
        if t < 0.0 { return self.points.first().unwrap().1; }
        if t > 1.0 { return self.points.last().unwrap().1; }
        for i in 0..self.points.len() {
            let point = self.points.get(i).unwrap();
            if t < point.0 {
                let prev_point = self.points.get(i - 1).unwrap();
                let (ax, ay) = (point.0 - prev_point.0, point.1 - prev_point.1);
                return ay / ax * (t - prev_point.0) + prev_point.1;
            }
        }
        self.points.last().unwrap().1
    }

}
