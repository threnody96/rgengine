use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ::resource::manager::ResourceManager;

pub struct PlaindataManager {
    cache: RefCell<HashMap<String, Rc<Vec<u8>>>>
}

impl PlaindataManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl ResourceManager<Vec<u8>> for PlaindataManager {

    fn generate(&self, _ctx: &mut Context, data: Vec<u8>) -> Result<Vec<u8>, String> {
        Ok(data)
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<Vec<u8>>>> {
        &self.cache
    }

}
