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
            fonts: RefCell::new(HashMap::new()),
            images: RefCell::new(HashMap::new()),
            strings: RefCell::new(HashMap::new()),
            jsons: RefCell::new(HashMap::new())
        }
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

    pub fn load_font(&self, ctx: &mut Context, path: String) -> Rc<Font> {
        let current = { self.fonts.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let font = self.generate_font(ctx, path.clone());
        self.fonts.borrow_mut().insert(path, font.clone());
        font
    }

    pub fn load_image(&self, ctx: &mut Context, path: String) -> Rc<Image> {
        let current = { self.images.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let image = self.generate_image(ctx, path.clone());
        self.images.borrow_mut().insert(path, image.clone());
        image
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

}
