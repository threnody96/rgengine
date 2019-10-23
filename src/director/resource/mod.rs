use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use ::resource::{ Resource };
use ::util::{ must };
use ggez::{ Context };
use ggez::graphics::{ Font, Image };
use serde_json::{ Value };

pub struct ResourceDirector {
    resource: Resource,
    plain_data: RefCell<HashMap<String, Rc<Vec<u8>>>>,
    preload_fonts: RefCell<Vec<String>>,
    preload_images: RefCell<Vec<String>>,
    fonts: RefCell<HashMap<String, Rc<Font>>>,
    images: RefCell<HashMap<String, Rc<Image>>>,
    strings: RefCell<HashMap<String, Rc<String>>>,
    jsons: RefCell<HashMap<String, Rc<Value>>>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resource: Resource::new(),
            plain_data: RefCell::new(HashMap::new()),
            preload_fonts: RefCell::new(Vec::new()),
            preload_images: RefCell::new(Vec::new()),
            fonts: RefCell::new(HashMap::new()),
            images: RefCell::new(HashMap::new()),
            strings: RefCell::new(HashMap::new()),
            jsons: RefCell::new(HashMap::new())
        }
    }

    pub fn do_preload(&self, ctx: &mut Context) {
        self.do_preload_fonts(ctx);
        self.do_preload_images(ctx);
    }

    pub fn preload_font(&self, path: String) {
        let mut fonts = self.preload_fonts.borrow_mut();
        fonts.push(path);
    }

    pub fn preload_image(&self, path: String) {
        let mut images = self.preload_images.borrow_mut();
        images.push(path);
    }

    pub fn load_plain_data(&self, path: String) -> Rc<Vec<u8>> {
        let current = { self.plain_data.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = Rc::new(must(self.resource.load(&path)));
        self.plain_data.borrow_mut().insert(path, data.clone());
        data
    }

    pub fn load_string(&self, path: String) -> Rc<String> {
        let current = { self.strings.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = self.load_plain_data(path.clone());
        let s = Rc::new(must(String::from_utf8(data.as_ref().clone())));
        self.strings.borrow_mut().insert(path, s.clone());
        s
    }

    pub fn load_json(&self, path: String) -> Rc<Value> {
        let current = { self.jsons.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let s = self.load_string(path.clone());
        let v: Rc<Value> = Rc::new(must(serde_json::from_str(s.as_ref().as_ref())));
        self.jsons.borrow_mut().insert(path, v.clone());
        v
    }

    pub fn load_font(&self, path: String) -> Rc<Font> {
        let current = self.fonts.borrow().get(&path).cloned();
        must(current.ok_or(format!("font not found: {}", path)))
    }

    pub fn load_image(&self, path: String) -> Rc<Image> {
        let current = self.images.borrow().get(&path).cloned();
        must(current.ok_or(format!("image not found: {}", path)))
    }

    fn generate_image(&self, ctx: &mut Context, path: String) -> Rc<Image> {
        let data = self.load_plain_data(path.clone());
        let i = must(image::load_from_memory(data.as_ref().as_slice()));
        let rgba = i.to_rgba();
        let mut pixel_data: Vec<u8> = Vec::new();
        rgba.pixels().into_iter().for_each(|item| pixel_data.extend(item.data.to_vec()));
        Rc::new(must(Image::from_rgba8(ctx, rgba.width() as u16, rgba.height() as u16, pixel_data.as_slice())))
    }

    fn generate_font(&self, ctx: &mut Context, path: String) -> Rc<Font> {
        if &path == "" {
            Rc::new(Font::default())
        } else {
            let data = self.load_plain_data(path);
            Rc::new(must(Font::new_glyph_font_bytes(ctx, data.as_ref().as_slice())))
        }
    }

    fn do_preload_fonts(&self, ctx: &mut Context) {
        let mut fonts = self.fonts.borrow_mut();
        let pathes = { self.preload_fonts.borrow().clone() };
        for path in pathes {
            if fonts.get(&path).is_none() {
                fonts.insert(path.clone(), self.generate_font(ctx, path.clone()));
            }
        }
        self.preload_fonts.replace(Vec::new());
    }

    fn do_preload_images(&self, ctx: &mut Context) {
        let mut images = self.images.borrow_mut();
        let pathes = { self.preload_images.borrow().clone() };
        for path in pathes {
            if images.get(&path).is_none() {
                images.insert(path.clone(), self.generate_image(ctx, path.clone()));
            }
        }
        self.preload_images.replace(Vec::new());
    }

}
