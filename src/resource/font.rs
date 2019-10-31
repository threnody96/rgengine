use ::resource::{ ResourceKey };

#[derive(Clone)]
pub struct RFont {
    key: ResourceKey
}

impl RFont {

    pub fn new(key: &ResourceKey) -> Self {
        Self { key: key.clone() }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

}

