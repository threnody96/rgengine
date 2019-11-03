use ::util::{ FuzzyArg };
use ::util::parameter::{ Size, TextureQuery };
use ::resource::{ ResourceKey };

#[derive(Clone)]
pub struct Texture {
    key: ResourceKey,
    info: TextureQuery
}

impl Texture {

    pub fn new<A, B>(key: A, info: B) -> Self
    where
        A: FuzzyArg<ResourceKey>,
        B: FuzzyArg<TextureQuery>
    {
        Self {
            key: key.take(),
            info: info.take()
        }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

    pub fn size(&self) -> Size {
        Size::new(self.info.width, self.info.height)
    }

}

