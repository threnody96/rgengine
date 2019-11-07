use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct BezierEase {
    action: Rc<dyn ActionLike>,
    points: Vec<(f32, f32)>
}

impl BezierEase {

    pub fn create(action: Rc<dyn ActionLike>, mut points: Vec<(f32, f32)>) -> Rc<ParentAction<BezierEase>> {
        let mut p: Vec<(f32, f32)> = vec!((0.0, 0.0));
        p.append(&mut points);
        p.push((1.0, 1.0));
        ParentAction::create(|| {
            Self {
                action: action.clone(),
                points: p.clone()
            }
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

    fn calc_bezier_curve(points: &Vec<(f32, f32)>, t: f32) -> (f32, f32) {
        let mut result: (f32, f32) = (0.0, 0.0);
        for i in 0i32..(points.len() as i32) {
            let n = points.len() as i32 - 1;
            let f = Self::factorial(n) / (Self::factorial(i) * Self::factorial(n - i));
            let j = (f as f32) * t.powi(i) * (1.0 - t).powi(n - i);
            let p = points.get(i as usize).unwrap();
            result.0 += p.0 * j;
            result.1 += p.1 * j;
        }
        result
    }

}

impl ParentActionDelegate for BezierEase {

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        let points = self.points.clone();
        self.action.run(node, &Some(Box::new(move |f| {
            Self::calc_bezier_curve(&points, f).1
        })))
    }

}
