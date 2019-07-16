use super::storage::Storage;
use std::collections::HashMap;
use std::rc::Rc;

pub trait ResourceLoader {
    fn load(&self, storage: Rc<Box<Storage>>, path: &str) -> Result<Vec<u8>, String>;
}

pub struct ResourceManager<R> where R: ResourceLoader {
    cache: HashMap<String, Rc<Vec<u8>>>,
    loader: Rc<R>,
}

impl<R> ResourceManager<R> where R: ResourceLoader {

    pub fn new(loader: Rc<R>) -> Self {
        Self { cache: HashMap::new(), loader: loader }
    }

    pub fn loader(&self) -> Rc<R> {
        self.loader.clone()
    }

    pub fn load(&mut self, storage: Rc<Box<Storage>>, path: &str) -> Result<Rc<Vec<u8>>, String> {
        let cache_key = Self::generate_cache_key(&storage.name(), path);
        let data = self.cache.get(&cache_key).cloned();
        match data {
            None => {
                let resource = Rc::new(self.loader.load(storage, path)?);
                self.cache.insert(cache_key, resource.clone());
                Ok(resource)
            },
            Some(d) => { Ok(d) }
        }
    }

    fn generate_cache_key(storage_name: &str, path: &str) -> String {
        format!("{}/{}", storage_name, path)
    }

}

pub mod texture;
pub mod plaindata;
