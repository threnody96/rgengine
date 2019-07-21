use std::rc::Rc;
use std::collections::HashMap;
use ggez::Context;
use ggez::graphics::Image;
use ::util::unwrap;
use ::resource::manager::ResourceManager;
use ::resource::storage::Storage;

pub struct Resource {
    managers: HashMap<String, Box<ResourceManager>>
}

impl Resource {

    pub fn new(storages: Vec<Box<Storage>>) -> Self {
        let mut map: HashMap<String, Box<ResourceManager>> = HashMap::new();
        for storage in storages {
            map.insert((*storage).name(), Box::new(ResourceManager::new(storage)));
        }
        Self { managers: map }
    }

    pub fn load_image(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<Image> {
        let manager = unwrap(self.manager(name));
        unwrap(manager.load_image(ctx, path))
    }

    pub fn load_plaindata(&self, name: &str, path: &str) -> Rc<Vec<u8>> {
        let manager = unwrap(self.manager(name));
        unwrap(manager.load_plaindata(path))
    }

    fn manager(&self, name: &str) -> Result<&Box<ResourceManager>, String> {
        self.managers.get(name).ok_or(format!("unknown manager: {}", name))
    }

}
