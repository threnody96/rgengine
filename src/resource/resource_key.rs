use ::resource::{ ResourceType };

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ResourceKey {
    path: String,
    rt: ResourceType
}

impl ResourceKey {

    pub fn new<A>(path: A, rt: ResourceType) -> Self
    where A: Into<String>
    {
        Self {
            path: path.into(),
            rt: rt
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

}

impl From<&ResourceKey> for ResourceKey {

    fn from(f: &ResourceKey) -> ResourceKey {
        f.clone()
    }

}
