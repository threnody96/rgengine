use ::util::{ FuzzyArg };
pub use sdl2::rect::{ Rect };

impl FuzzyArg<Rect> for Rect {

    fn take(&self) -> Rect {
        self.clone()
    }

}

impl FuzzyArg<Rect> for &Rect {

    fn take(&self) -> Rect {
        (*self).clone()
    }

}

impl FuzzyArg<Rect> for (i32, i32, u32, u32) {

    fn take(&self) -> Rect {
        Rect::new(self.0, self.1, self.2, self.3)
    }

}
