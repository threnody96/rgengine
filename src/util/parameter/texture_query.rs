use ::util::{ FuzzyArg };
pub use sdl2::render::{ TextureQuery };

impl FuzzyArg<TextureQuery> for TextureQuery {

    fn take(&self) -> TextureQuery {
        self.clone()
    }

}

impl FuzzyArg<TextureQuery> for &TextureQuery {

    fn take(&self) -> TextureQuery {
        (*self).clone()
    }

}
