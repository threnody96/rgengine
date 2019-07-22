use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ggez::Context;
use ::resource::storage::Storage;
use ::util::unwrap;

pub trait ResourceManager<T> {

    fn load(&self, ctx: &mut Context, storage: &Box<dyn Storage>, path: &str) -> Rc<T> {
        let fullpath = format!("{}/{}", storage.name(), path);
        let c = self.load_cache(&fullpath);
        if c.is_some() { return c.unwrap(); }
        let data = Rc::new(unwrap(self.generate(ctx, unwrap(storage.load(path)))));
        self.set_cache(&fullpath, data.clone());
        data
    }

    fn load_cache(&self, path: &str) -> Option<Rc<T>> {
        let cache = self.cache().borrow();
        cache.get(path).cloned()
    }

    fn set_cache(&self, path: &str, data: Rc<T>) {
        let mut cache = self.cache().borrow_mut();
        cache.insert(path.to_owned(), data);
    }

    fn generate(&self, ctx: &mut Context, data: Vec<u8>) -> Result<T, String>;

    fn cache(&self) -> &RefCell<HashMap<String, Rc<T>>>;

}
