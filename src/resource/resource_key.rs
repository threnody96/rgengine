use ::util::{ FuzzyArg };
use ::resource::{ ResourceType };

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ResourceKey {
    path: String,
    rt: ResourceType
}

impl ResourceKey {

    pub fn new<A>(path: A, rt: ResourceType) -> Self
    where A: FuzzyArg<String>
    {
        Self {
            path: path.take(),
            rt: rt
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

}

impl FuzzyArg<ResourceKey> for ResourceKey {

    fn take(&self) -> ResourceKey {
        self.clone()
    }

}

impl FuzzyArg<ResourceKey> for &ResourceKey {

    fn take(&self) -> ResourceKey {
        (*self).clone()
    }

}
