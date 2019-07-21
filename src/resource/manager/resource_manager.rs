use ::resource::storage::Storage;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use ggez::Context;
use ggez::graphics::Image;

pub struct ResourceManager {
    plaindata_cache: RefCell<HashMap<String, Rc<Vec<u8>>>>,
    image_cache: RefCell<HashMap<String, Rc<Image>>>,
    storage: Box<dyn Storage>,
}

impl ResourceManager {

    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self {
            plaindata_cache: RefCell::new(HashMap::new()),
            image_cache: RefCell::new(HashMap::new()),
            storage: storage
        }
    }

    pub fn load_image(&self, ctx: &mut Context, path: &str) -> Result<Rc<Image>, String> {
        let cache_data = self.load_by_image_cache(path);
        if cache_data.is_some() { return Ok(cache_data.unwrap()); }
        let plain_resource = self.load_plaindata(path)?;
        let resource = Rc::new(self.load_image_by_plaindata(ctx, (*plain_resource).to_owned())?);
        self.cache_image(path, resource.clone());
        Ok(resource)
    }

    pub fn load_plaindata(&self, path: &str) -> Result<Rc<Vec<u8>>, String> {
        let cache_data = self.load_by_plaindata_cache(path);
        if cache_data.is_some() { return Ok(cache_data.unwrap()); }
        let resource = Rc::new((*self.storage).load(path)?);
        self.cache_plaindata(path, resource.clone());
        Ok(resource)
    }

    fn load_image_by_plaindata(&self, ctx: &mut Context, data: Vec<u8>) -> Result<Image, String> {
        let i = image::load_from_memory(data.as_slice()).map_err(|e| e.to_string())?;
        let rgba = i.to_rgba();
        let mut pixel_datas = Vec::new();
        rgba.pixels().into_iter().for_each(|item| pixel_datas.extend(item.data.to_vec()));
        Image::from_rgba8(ctx, rgba.width() as u16, rgba.height() as u16, pixel_datas.as_slice()).map_err(|e| e.to_string())
    }

    fn load_by_plaindata_cache(&self, path: &str) -> Option<Rc<Vec<u8>>> {
        let cache = self.plaindata_cache.borrow();
        cache.get(path).cloned()
    }

    fn load_by_image_cache(&self, path: &str) -> Option<Rc<Image>> {
        let cache = self.image_cache.borrow();
        cache.get(path).cloned()
    }

    fn cache_plaindata(&self, path: &str, resource: Rc<Vec<u8>>) {
        let mut cache = self.plaindata_cache.borrow_mut();
        cache.insert(path.to_owned(), resource);
    }

    fn cache_image(&self, path: &str, resource: Rc<Image>) {
        let mut cache = self.image_cache.borrow_mut();
        cache.insert(path.to_owned(), resource);
    }

}
