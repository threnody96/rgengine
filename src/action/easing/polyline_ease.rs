use std::rc::Rc;
use ::node::{ NodeLike };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };

pub struct PolylineEase {
    action: Rc<dyn ActionLike>,
    points: Vec<(f32, f32)>
}

impl PolylineEase {

    pub fn create(action: Rc<dyn ActionLike>, mut points: Vec<(f32, f32)>) -> Rc<ParentAction<PolylineEase>> {
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

    fn calc_progress(points: &Vec<(f32, f32)>, t: f32) -> f32 {
        if t < 0.0 { return points.first().unwrap().1; }
        if t > 1.0 { return points.last().unwrap().1; }
        for i in 0..points.len() {
            let point = points.get(i).unwrap();
            if t < point.0 {
                let prev_point = points.get(i - 1).unwrap();
                let (ax, ay) = (point.0 - prev_point.0, point.1 - prev_point.1);
                return ay / ax * (t - prev_point.0) + prev_point.1;
            }
        }
        points.last().unwrap().1
    }

}

impl ParentActionDelegate for PolylineEase {

    fn run(&self, node: Rc<dyn NodeLike>, easing: &Option<Box<Fn(f32) -> f32>>) -> ActionStatus {
        let points = self.points.clone();
        self.action.run(node, &Some(Box::new(move |f| {
            Self::calc_progress(&points, f)
        })))
    }

}
