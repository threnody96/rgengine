use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use ::resource::{ Resource };
use ::util::{ must };
use ggez::{ Context };
use ggez::graphics::{ Font, Image };

pub struct ResourceDirector {
    resource: Resource,
    plain_data: RefCell<HashMap<String, Rc<Vec<u8>>>>,
    fonts: RefCell<HashMap<String, Rc<Font>>>,
    images: RefCell<HashMap<String, Rc<Image>>>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resource: Resource::new(),
            plain_data: RefCell::new(HashMap::new()),
            fonts: RefCell::new(HashMap::new()),
            images: RefCell::new(HashMap::new())
        }
    }

    pub fn load_plain_data(&self, path: String) -> Rc<Vec<u8>> {
        let current = { self.plain_data.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = Rc::new(must(self.resource.load(&path)));
        self.plain_data.borrow_mut().insert(path, data.clone());
        data
    }

    pub fn load_font(&self, ctx: &mut Context, path: String) -> Rc<Font> {
        let current = { self.fonts.borrow().get(&path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = self.load_plain_data(path.clone());
        let font = Rc::new(must(Font::new_glyph_font_bytes(ctx, data.as_ref().as_slice())));
        self.fonts.borrow_mut().insert(path, font.clone());
        font
    }

    pub fn load_image(&self, ctx: &mut Context, path: String) -> Rc<Image> {
        let data = self.load_plain_data(path.clone());
        let i = must(image::load_from_memory(data.as_ref().as_slice()));
        let rgba = i.to_rgba();
        let mut pixel_data: Vec<u8> = Vec::new();
        rgba.pixels().into_iter().for_each(|item| pixel_data.extend(item.data.to_vec()));
        let image = Rc::new(must(Image::from_rgba8(ctx, rgba.width() as u16, rgba.height() as u16, pixel_data.as_slice())));
        self.images.borrow_mut().insert(path, image.clone());
        image
    }

}
