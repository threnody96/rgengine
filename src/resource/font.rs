use ::resource::{ ResourceKey };

#[derive(Clone)]
pub struct Font {
    key: ResourceKey
}

impl Font {

    pub fn new<A>(key: A) -> Self
    where A: Into<ResourceKey>
    {
        Self { key: key.into() }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

}

