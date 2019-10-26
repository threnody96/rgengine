use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use ::resource::{ Resource };
use ::util::{ must, canvas };
use sdl2::render::{ Texture };
use sdl2::surface::{ Surface };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };

pub struct ResourceDirector {
    resource: Resource,
    plain_datas: RefCell<HashMap<String, Rc<Vec<u8>>>>,
    textures: RefCell<HashMap<String, Rc<Texture>>>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resource: Resource::new(),
            plain_datas: RefCell::new(HashMap::new()),
            textures: RefCell::new(HashMap::new())
        }
    }

    pub fn load_plain_data(&self, path: &str) -> Rc<Vec<u8>> {
        let current = { self.plain_datas.borrow().get(path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = Rc::new(must(self.resource.load(path)));
        self.plain_datas.borrow_mut().insert(path.to_owned(), data.clone());
        data
    }

    pub fn load_texture(&self, path: &str) -> Rc<Texture> {
        let current = { self.textures.borrow().get(path).cloned() };
        if current.is_some() { return current.unwrap(); }
        let data = self.load_plain_data(path);
        let texture = Rc::new(self.generate_texture(data));
        self.textures.borrow_mut().insert(path.to_owned(), texture.clone());
        texture
    }

    fn generate_texture(&self, data: Rc<Vec<u8>>) -> Texture {
        let rwops = must(RWops::from_bytes(data.as_slice()));
        let surface = must(rwops.load());
        let tc = canvas(|c| c.texture_creator());
        must(tc.create_texture_from_surface(surface))
    }

}