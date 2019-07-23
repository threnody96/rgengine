use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ::resource::manager::ResourceManager;

pub struct TextManager {
    cache: RefCell<HashMap<String, Rc<String>>>
}

impl TextManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl ResourceManager<String> for TextManager {

    fn generate(&self, _ctx: &mut Context, data: Vec<u8>) -> Result<String, String> {
        String::from_utf8(data).map(|e| e.to_owned()).map_err(|e| e.to_string())
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<String>>> {
        &self.cache
    }

}
