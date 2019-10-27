use std::rc::Rc;
use sdl2::render::{ Texture, TextureCreator };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };
use sdl2::video::{ WindowContext };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RTexture {
    key: String
}

impl RTexture {

    pub fn new(key: &str) -> Self {
        Self { key: key.to_owned() }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

}

