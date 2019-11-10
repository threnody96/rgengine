use std::ops::{ Deref };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Point {
    point: sdl2::rect::Point
}

impl Point {

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            point: sdl2::rect::Point::new(x, y)
        }
    }

    pub fn distance<A>(&self, other: A) -> f32
    where A: Into<Point>
    {
        let o = other.into();
        let x = (self.x() as f32, o.x() as f32);
        let y = (self.y() as f32, o.y() as f32);
        ((x.0 - x.1).powi(2) + (y.0 - y.1).powi(2)).sqrt()
    }

}

impl Deref for Point {

    type Target = sdl2::rect::Point;

    fn deref(&self) -> &Self::Target {
        &self.point
    }

}

impl Into<sdl2::rect::Point> for Point {

    fn into(self) -> sdl2::rect::Point {
        self.point
    }

}

impl From<sdl2::rect::Point> for Point {

    fn from(f: sdl2::rect::Point) -> Self {
        Self::new(f.x(), f.y())
    }

}

impl From<&sdl2::rect::Point> for Point {

    fn from(f: &sdl2::rect::Point) -> Self {
        Self::from(f.clone())
    }

}

impl From<&Point> for Point {

    fn from(f: &Point) -> Point {
        f.clone()
    }

}

