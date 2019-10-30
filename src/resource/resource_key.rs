use ::resource::{ ResourceType };

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ResourceKey {
    path: String,
    rt: ResourceType
}

impl ResourceKey {

    pub fn new(path: &str, rt: ResourceType) -> Self {
        Self {
            path: path.to_owned(),
            rt: rt
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

}
