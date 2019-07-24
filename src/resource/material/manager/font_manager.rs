use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ggez::graphics::{ Font };
use ::resource::material::manager::MaterialManager;

pub struct FontManager {
    cache: RefCell<HashMap<String, Rc<Font>>>
}

impl FontManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl MaterialManager<Font> for FontManager {

    fn generate(&self, ctx: &mut Context, data: Vec<u8>) -> Result<Font, String> {
        Font::new_glyph_font_bytes(ctx, data.as_slice()).map_err(|e| e.to_string())
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<Font>>> {
        &self.cache
    }

}
