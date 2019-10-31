use ::util::{ Size };
use ::resource::{ ResourceKey };
use sdl2::render::{ TextureQuery };

#[derive(Clone)]
pub struct RTexture {
    key: ResourceKey,
    info: TextureQuery
}

impl RTexture {

    pub fn new(key: &ResourceKey, info: &TextureQuery) -> Self {
        Self {
            key: key.clone(),
            info: info.clone()
        }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

    pub fn size(&self) -> Size {
        Size::new(self.info.width, self.info.height)
    }

}

