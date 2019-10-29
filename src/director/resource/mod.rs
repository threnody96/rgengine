use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;
use ::node::{ LabelOption };
use ::resource::{ RTexture, RFont };
use ::util::{ render, Must };
use serde_json::Value;
use sdl2::ttf::FontStyle;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ResourceType {
    Texture,
    Font(u16, FontStyle)
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ResourceKey {
    path: String,
    rt: ResourceType
}

pub struct ResourceDirector {
    aliases: HashMap<String, String>,
    resources: HashMap<ResourceKey, Rc<dyn Any>>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            aliases: HashMap::new()
        }
    }

    fn get_resource<T>(&self, key: &ResourceKey) -> Option<Rc<T>> where T: Any {
        if let Some(r) = self.resources.get(key) {
            match r.clone().downcast::<T>() {
                Ok(t) => { Some(t) }
                Err(e) => { None }
            }
        } else {
            None
        }
    }

    pub fn add_alias(&mut self, name: &str, path: &str) {
        self.aliases.insert(name.to_owned(), path.to_owned());
    }

    fn resolve_path(&self, name: &str) -> String {
        match self.aliases.get(name) {
            Some(path) => { path.clone() },
            None => { name.to_owned() }
        }
    }

    pub fn load_plain_data(&mut self, path: &str) -> Rc<Vec<u8>> {
        let p = self.resolve_path(path);
        render(|r| r.load_plain_data(&p))
    }

    pub fn load_string(&mut self, path: &str) -> Rc<String> {
        let p = self.resolve_path(path);
        render(|r| r.load_string(&p))
    }

    pub fn load_json(&mut self, path: &str) -> Rc<Value> {
        let p = self.resolve_path(path);
        render(|r| r.load_json(&p))
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<RTexture> {
        let p = self.resolve_path(path);
        let key = ResourceKey {
            path: p.clone(),
            rt: ResourceType::Texture
        };
        if let Some(texture) = self.get_resource::<RTexture>(&key) {
            texture
        } else {
            let k = render(|r| r.load_texture(&p));
            self.resources.insert(key, k.clone());
            k
        }
    }

    pub fn load_font(&mut self, option: &LabelOption) -> Rc<RFont> {
        let p = self.resolve_path(&option.path);
        let key = ResourceKey {
            path: p.clone(),
            rt: ResourceType::Font(option.point, option.style)
        };
        if let Some(font) = self.get_resource::<RFont>(&key) {
            font
        } else {
            let k = render(|r| r.load_font(&p, option.point, option.style));
            self.resources.insert(key, k.clone());
            k
        }
    }

}

