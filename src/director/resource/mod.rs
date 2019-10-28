use std::rc::Rc;
use std::collections::HashMap;
use ::resource::{ RTexture, RFont };
use ::util::{ must, render };
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
    resources: HashMap<ResourceKey, String>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resources: HashMap::new()
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

    pub fn load_texture(&mut self, path: &str) -> RTexture {
        let key = ResourceKey {
            path: path.to_owned(),
            rt: ResourceType::Texture
        };
        if let Some(texture) = self.resources.get(&key) {
            RTexture::new(texture.as_ref())
        } else {
            let k = render(|r| r.load_texture(path));
            self.resources.insert(key, k.clone());
            RTexture::new(k.as_ref())
        }
    }

    pub fn load_font(&mut self, path: &str, point: u16, style: FontStyle) -> RFont {
        let key = ResourceKey {
            path: path.to_owned(),
            rt: ResourceType::Font(point, style)
        };
        if let Some(font) = self.resources.get(&key) {
            RFont::new(font.as_ref())
        } else {
            let k = render(|r| r.load_font(path, point, style));
            self.resources.insert(key, k.clone());
            RFont::new(k.as_ref())
        }
    }

}

