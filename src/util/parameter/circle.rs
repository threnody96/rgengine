use std::cmp::min;
use ::util::parameter::{ Point, Rect };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Circle {
    center: Point,
    diameter: u32
}

impl Circle {

    pub fn new<T>(center: T, diameter: u32) -> Self
    where T: Into<Point>
    {
        Self {
            center: center.into(),
            diameter: diameter
        }
    }

    pub fn has_intersection(&self, other: &Circle) -> bool {
        let center_distance = self.center.distance(&other.center) as u32;
        let radius = self.diameter / 2 + self.diameter / 2;
        radius >= center_distance
    }

    pub fn has_rect_intersection(&self, other: &Rect) -> bool {
        let radius = self.diameter / 2;
        let rect1 = Rect::new(other.x(), other.y() - radius as i32, other.width(), other.height() + radius);
        if rect1.contains_point(*self.center) { return true; }
        let rect2 = Rect::new(other.x() - radius as i32, other.y(), other.width() + radius, other.height());
        if rect2.contains_point(*self.center) { return true; }
        let points = vec!(other.top_left(), other.top_right(), other.bottom_left(), other.bottom_right());
        for p in &points {
            if self.contains_point(p) { return true; }
        }
        false
    }

    pub fn contains_point<A>(&self, point: A) -> bool
    where A: Into<Point>
    {
        let p = point.into();
        let radius = self.diameter / 2;
        self.center.distance(p) as u32 <= radius
    }

    pub fn center(&self) -> Point {
        self.center.clone()
    }

    pub fn diameter(&self) -> u32 {
        self.diameter
    }

}

impl From<Rect> for Circle {

    fn from(f: Rect) -> Self {
        Self::new(f.center(), min(f.width(), f.height()))
    }

}

impl From<&Rect> for Circle {

    fn from(f: &Rect) -> Self {
        Self::from(f.clone())
    }

}