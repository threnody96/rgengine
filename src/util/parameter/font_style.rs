use ::util::{ FuzzyArg };
pub use sdl2::ttf::{ FontStyle };

impl FuzzyArg<FontStyle> for FontStyle {

    fn take(&self) -> FontStyle {
        self.clone()
    }

}

impl FuzzyArg<FontStyle> for &FontStyle {

    fn take(&self) -> FontStyle {
        (*self).clone()
    }

}
