use std::rc::Rc;
use std::collections::HashMap;
use std::borrow::Borrow;
use ggez::Context;
use ggez::audio::{ SoundData };
use ggez::graphics::{ Font, Image };
use serde_json::value::Value;
use serde::de::DeserializeOwned;
use ::util::unwrap;
use ::resource::manager::{
    TextManager, ImageManager, FontManager, PlaindataManager, SoundManager, ResourceManager, JsonManager
};
use ::resource::storage::Storage;

pub struct Resource {
    plaindata: PlaindataManager,
    text: TextManager,
    image: ImageManager,
    font: FontManager,
    sound: SoundManager,
    json: JsonManager,
    storages: HashMap<String, Box<dyn Storage>>
}

impl Resource {

    pub fn new(storages: Vec<Box<dyn Storage>>) -> Self {
        let mut map: HashMap<String, Box<dyn Storage>> = HashMap::new();
        for storage in storages { map.insert((*storage).name(), storage); }
        Self {
            plaindata: PlaindataManager::new(),
            text: TextManager::new(),
            image: ImageManager::new(),
            font: FontManager::new(),
            sound: SoundManager::new(),
            json: JsonManager::new(),
            storages: map
        }
    }

    pub fn load_struct<T>(&self, ctx: &mut Context, name: &str, path: &str) -> T
    where T: DeserializeOwned
    {
        let json = self.load_json(ctx, name, path);
        let borrowed: &Value = json.borrow();
        unwrap(serde_json::from_value(borrowed.clone()))
    }

    pub fn load_json(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<Value> {
        self.json.load(ctx, self.storage(name), path)
    }

    pub fn load_text(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<String> {
        self.text.load(ctx, self.storage(name), path)
    }

    pub fn load_sound(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<SoundData> {
        self.sound.load(ctx, self.storage(name), path)
    }

    pub fn load_font(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<Font> {
        self.font.load(ctx, self.storage(name), path)
    }

    pub fn load_image(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<Image> {
        self.image.load(ctx, self.storage(name), path)
    }

    pub fn load_plaindata(&self, ctx: &mut Context, name: &str, path: &str) -> Rc<Vec<u8>> {
        self.plaindata.load(ctx, self.storage(name), path)
    }

    fn storage(&self, name: &str) -> &Box<dyn Storage> {
        unwrap(self.storages.get(name).ok_or(format!("unknown storage: {}", name)))
    }

}
