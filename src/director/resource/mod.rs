use std::rc::Rc;
use std::collections::HashMap;
use ::node::{ LabelOption };
use ::resource::{ RTexture, RFont, Storage, ResourceType, ResourceKey };
use ::util::{ context };
use serde_json::Value;
use sdl2::render::{ Texture };
use sdl2::ttf::{ Font };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };
use uuid::Uuid;

pub struct ResourceDirector<'a> {
    storage: Storage,
    aliases: HashMap<String, String>,
    plain_datas: HashMap<ResourceKey, Rc<Vec<u8>>>,
    strings: HashMap<ResourceKey, Rc<String>>,
    jsons: HashMap<ResourceKey, Rc<Value>>,
    textures: HashMap<ResourceKey, Rc<Texture<'a>>>,
    fonts: HashMap<ResourceKey, Rc<Font<'a, 'a>>>,
    render_caches: HashMap<ResourceKey, Rc<Texture<'a>>>,
}

impl <'a> ResourceDirector<'a> {

    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            aliases: HashMap::new(),
            plain_datas: HashMap::new(),
            strings: HashMap::new(),
            jsons: HashMap::new(),
            textures: HashMap::new(),
            fonts: HashMap::new(),
            render_caches: HashMap::new(),
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

    fn generate_resource_key(&self, path: &str, rt: ResourceType) -> ResourceKey {
        let p = self.resolve_path(path);
        ResourceKey::new(&p, rt)
    }

    pub fn load_plain_data(&mut self, path: &str) -> Rc<Vec<u8>> {
        let resource_key = self.generate_resource_key(path, ResourceType::PlainData);
        if let Some(data) = self.plain_datas.get(&resource_key) {
            data.clone()
        } else {
            let data = Rc::new(self.storage.load(&resource_key.path()).unwrap());
            self.plain_datas.insert(resource_key, data.clone());
            data
        }
    }

    pub fn load_string(&mut self, path: &str) -> Rc<String> {
        let resource_key = self.generate_resource_key(path, ResourceType::String);
        if let Some(data) = self.strings.get(&resource_key) {
            data.clone()
        } else {
            let data = self.load_plain_data(&resource_key.path());
            let s = Rc::new(String::from_utf8(data.as_ref().clone()).unwrap());
            self.strings.insert(resource_key, s.clone());
            s
        }
    }

    pub fn load_json(&mut self, path: &str) -> Rc<Value> {
        let resource_key = self.generate_resource_key(path, ResourceType::Json);
        if let Some(current) = self.jsons.get(&resource_key) {
            current.clone()
        } else {
            let data = self.load_string(&resource_key.path());
            let json: Value = serde_json::from_str(data.as_str()).unwrap();
            let j = Rc::new(json);
            self.jsons.insert(resource_key, j.clone());
            j
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<RTexture> {
        let resource_key = self.generate_resource_key(path, ResourceType::Texture);
        if let Some(current) = self.textures.get(&resource_key) {
            let query = current.query();
            Rc::new(RTexture::new(&resource_key, &query))
        } else {
            let data = self.load_plain_data(&resource_key.path());
            let rwops = RWops::from_bytes(data.as_slice()).unwrap();
            let surface = rwops.load().unwrap();
            let texture = Rc::new(context(|c| c.texture_creator.create_texture_from_surface(surface)).unwrap());
            let query = texture.query();
            self.textures.insert(resource_key.clone(), texture);
            Rc::new(RTexture::new(&resource_key, &query))
        }
    }

    pub fn load_texture_from_resource_key(&self, key: Rc<RTexture>) -> Rc<Texture<'a>> {
        let resource_key = key.key();
        self.textures.get(&resource_key).unwrap().clone()
    }

    pub fn load_font(&mut self, option: &LabelOption) -> Rc<RFont> {
        let resource_key = self.generate_resource_key(
            &option.path,
            ResourceType::Font(option.point, option.style)
        );
        if let Some(_) = self.fonts.get(&resource_key) {
            Rc::new(RFont::new(&resource_key))
        } else {
            let font_data = self.load_plain_data(&resource_key.path());
            context(|c| c.add_font_data(&resource_key, font_data));
            let data = context(|c| c.get_font_data(&resource_key));
            let rwops = RWops::from_bytes(data).unwrap();
            let mut font = context(|c| c.ttf_context.load_font_from_rwops(rwops, option.point)).unwrap();
            font.set_style(option.style);
            self.fonts.insert(resource_key.clone(), Rc::new(font));
            Rc::new(RFont::new(&resource_key))
        }
    }

    pub fn load_font_from_resource_key(&self, key: Rc<RFont>) -> Rc<Font<'a, 'a>> {
        let resource_key = key.key();
        self.fonts.get(&resource_key).unwrap().clone()
    }

    pub fn set_render_cache(&mut self, sub_canvas: Rc<Texture<'a>>) -> ResourceKey {
        let key = ResourceKey::new(&Uuid::new_v4().to_string(), ResourceType::RenderCache);
        self.render_caches.insert(key.clone(), sub_canvas);
        key
    }

    pub fn get_render_cache(&self, key: &ResourceKey) -> Rc<Texture<'a>> {
        self.render_caches.get(key).unwrap().clone()
    }

    pub fn destroy_render_cache(&mut self, key: &ResourceKey) {
        self.render_caches.remove(key);
    }

}

