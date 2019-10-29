use std::rc::Rc;
use ::util::{ Size };
use sdl2::render::{ Texture, TextureCreator, TextureQuery };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };
use sdl2::video::{ WindowContext };

#[derive(Clone)]
pub struct RTexture {
    key: String,
    info: TextureQuery
}

impl RTexture {

    pub fn new(key: &str, info: &TextureQuery) -> Self {
        Self {
            key: key.to_owned(),
            info: info.clone()
        }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn size(&self) -> Size {
        Size::new(self.info.width, self.info.height)
    }

}

