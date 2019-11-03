use ::util::{ FuzzyArg };
use ::resource::{ ResourceKey };

#[derive(Clone)]
pub struct Font {
    key: ResourceKey
}

impl Font {

    pub fn new<A>(key: A) -> Self
    where A: FuzzyArg<ResourceKey>
    {
        Self { key: key.take() }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

}

