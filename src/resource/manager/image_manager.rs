use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ggez::graphics::{ Image };
use ::resource::manager::ResourceManager;

pub struct ImageManager {
    cache: RefCell<HashMap<String, Rc<Image>>>
}

impl ImageManager {

    pub fn new() -> Self {
        Self { cache: RefCell::new(HashMap::new()) }
    }

}

impl ResourceManager<Image> for ImageManager {

    fn generate(&self, ctx: &mut Context, data: Vec<u8>) -> Result<Image, String> {
        let i = image::load_from_memory(data.as_slice()).map_err(|e| e.to_string())?;
        let rgba = i.to_rgba();
        let mut pixel_datas = Vec::new();
        rgba.pixels().into_iter().for_each(|item| pixel_datas.extend(item.data.to_vec()));
        Image::from_rgba8(ctx, rgba.width() as u16, rgba.height() as u16, pixel_datas.as_slice()).map_err(|e| e.to_string())
    }

    fn cache(&self) -> &RefCell<HashMap<String, Rc<Image>>> {
        &self.cache
    }

}
