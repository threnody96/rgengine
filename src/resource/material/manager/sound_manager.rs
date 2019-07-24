use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ggez::audio::{ SoundData };
use ::resource::material::manager::MaterialManager;

pub struct SoundManager {
    cache: RefCell<HashMap<String, Rc<SoundData>>>
}

impl SoundManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl MaterialManager<SoundData> for SoundManager {

    fn generate(&self, _ctx: &mut Context, data: Vec<u8>) -> Result<SoundData, String> {
        Ok(SoundData::from_bytes(data.as_slice()))
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<SoundData>>> {
        &self.cache
    }

}
