use ::util::{ FuzzyArg };
pub use sdl2::rect::{ Point };

impl FuzzyArg<Point> for Point {

    fn take(&self) -> Point {
        self.clone()
    }

}

impl FuzzyArg<Point> for &Point {

    fn take(&self) -> Point {
        (*self).clone()
    }

}

impl FuzzyArg<Point> for (i32, i32) {

    fn take(&self) -> Point {
        Point::new(self.0, self.1)
    }

}
