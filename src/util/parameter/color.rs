use ::util::{ FuzzyArg };
pub use sdl2::pixels::{ Color };

impl FuzzyArg<Color> for Color {

    fn take(&self) -> Color {
        self.clone()
    }

}

impl FuzzyArg<Color> for &Color {

    fn take(&self) -> Color {
        (*self).clone()
    }

}

impl FuzzyArg<Color> for (u8, u8, u8, u8) {

    fn take(&self) -> Color {
        Color::RGBA(self.0, self.1, self.2, self.3)
    }

}
