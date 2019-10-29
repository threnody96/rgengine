use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;
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
    resources: HashMap<ResourceKey, Rc<dyn Any>>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resources: HashMap::new()
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

    pub fn load_plain_data(&mut self, path: &str) -> Rc<Vec<u8>> {
        render(|r| r.load_plain_data(path))
    }

    pub fn load_string(&mut self, path: &str) -> Rc<String> {
        render(|r| r.load_string(path))
    }

    pub fn load_json(&mut self, path: &str) -> Rc<Value> {
        render(|r| r.load_json(path))
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<RTexture> {
        let key = ResourceKey {
            path: path.to_owned(),
            rt: ResourceType::Texture
        };
        if let Some(texture) = self.get_resource::<RTexture>(&key) {
            texture
        } else {
            let k = render(|r| r.load_texture(path));
            self.resources.insert(key, k.clone());
            k
        }
    }

    pub fn load_font(&mut self, path: &str, point: u16, style: FontStyle) -> Rc<RFont> {
        let key = ResourceKey {
            path: path.to_owned(),
            rt: ResourceType::Font(point, style)
        };
        if let Some(font) = self.get_resource::<RFont>(&key) {
            font
        } else {
            let k = render(|r| r.load_font(path, point, style));
            self.resources.insert(key, k.clone());
            k
        }
    }

}

