use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use serde_json::value::Value;
use ::resource::material::manager::MaterialManager;

pub struct JsonManager {
    cache: RefCell<HashMap<String, Rc<Value>>>
}

impl JsonManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl MaterialManager<Value> for JsonManager {

    fn generate(&self, _ctx: &mut Context, data: Vec<u8>) -> Result<Value, String> {
        let json = String::from_utf8(data).map_err(|e| e.to_string())?;
        Ok(json!(json))
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<Value>>> {
        &self.cache
    }

}
