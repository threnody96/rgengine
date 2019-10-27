use std::rc::Rc;
use std::collections::HashMap;
use ::resource::{ RTexture, RFont };
use ::util::{ must, load_texture };

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ResourceType {
    Texture,
    Font(u16)
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ResourceKey {
    path: String,
    rt: ResourceType
}

pub struct ResourceDirector {
    resources: HashMap<ResourceKey, String>
}

impl ResourceDirector {

    pub fn new() -> Self {
        Self {
            resources: HashMap::new()
        }
    }

    pub fn load_texture(&mut self, path: &str) -> RTexture {
        let key = ResourceKey {
            path: path.to_owned(),
            rt: ResourceType::Texture
        };
        if let Some(texture) = self.resources.get(&key) {
            RTexture::new(texture.as_ref())
        } else {
            let k = load_texture(path);
            self.resources.insert(key, k.clone());
            RTexture::new(k.as_ref())
        }
    }

    pub fn load_font(&mut self, path: String, point: u16) -> RFont {
        let key = ResourceKey {
            path: path,
            rt: ResourceType::Font(point)
        };
        let t = self.resources.get(&key);
        if t.is_some() { return RFont::new(&t.unwrap()); }
        RFont::new("hogehoge")
    }

}

// use std::cell::RefCell;
// use std::rc::Rc;
// use std::collections::HashMap;
// use ::resource::{ Resource };
// use ::util::{ must, canvas };
// use sdl2::render::{ Texture, TextureCreator };
// use sdl2::surface::{ Surface };
// use sdl2::rwops::{ RWops };
// use sdl2::image::{ ImageRWops };
// use sdl2::video::{ WindowContext };

// pub struct ResourceDirector<'a> {
//     // resource: Resource,
//     // texture_creator: TextureCreator<WindowContext>,
//     // plain_datas: HashMap<String, Vec<u8>>,
//     // rwops: HashMap<String, Rc<RWops<'a>>>,
//     // surfaces: HashMap<String, Rc<Surface<'a>>>,
//     // textures: HashMap<String, Rc<Texture<'a>>>
// }

// impl <'a> ResourceDirector<'a> {
//
//     // pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
//     //     Self {
//     //         resource: Resource::new(),
//     //         texture_creator: texture_creator,
//     //         plain_datas: HashMap::new(),
//     //         rwops: HashMap::new(),
//     //         surfaces: HashMap::new(),
//     //         textures: HashMap::new()
//     //     }
//     // }
//
//     // // pub fn set_texture_creator(&mut self, texture_creator: TextureCreator<WindowContext>) {
//     // //     self.texture_creator = Some(texture_creator);
//     // // }
//
//     // pub fn load_plain_data(&mut self, path: &str) -> &'a Vec<u8> {
//     //     let current = self.plain_datas.get(path);
//     //     if current.is_some() { return current.unwrap(); }
//     //     let data = must(self.resource.load(path));
//     //     self.plain_datas.insert(path.to_owned(), data);
//     //     self.plain_datas.get(path).unwrap()
//     // }
//
//     // pub fn load_rwops(&mut self, path: &str) -> Rc<RWops<'a>> {
//     //     let current = self.rwops.get(path).cloned();
//     //     if current.is_some() { return current.unwrap(); }
//     //     let data = self.load_plain_data(path);
//     //     let rwops = Rc::new(must(RWops::from_bytes(data.as_slice())));
//     //     self.rwops.insert(path.to_owned(), rwops.clone());
//     //     rwops
//     // }
//
//     // // pub fn load_surfaces(&mut self, path: &str) -> Rc<Surface<'a>> {
//     // //     let current = self.surfaces.get(path).cloned();
//     // //     if current.is_some() { return current.unwrap(); }
//     // //     let data = self.load_plain_data(path);
//     // //     let rwops = must(RWops::from_bytes(data.as_slice()));
//     // //     let surface = Rc::new(must(rwops.load()));
//     // //     self.surfaces.insert(path.to_owned(), surface.clone());
//     // //     surface
//     // // }
//
//     // // pub fn load_texture(&mut self, path: &str) {
//     // //     let data = self.load_plain_data(path);
//     // //     let rwops = must(RWops::from_bytes(data.as_slice()));
//     // //     let surface = must(rwops.load());
//     // //     let t = self.texture_creator.create_texture_from_surface(surface).unwrap();
//     // //     let texture = Rc::new(t);
//     // //     self.textures.insert(path.to_owned(), texture.clone());
//     // // }
//
//     // // pub fn load_texture(&self, path: &str) {
//     // //     let current = { self.textures.borrow().get(path).cloned() };
//     // //     if current.is_some() { return; }
//     // //     self.load_plain_data(path);
//     // //     let data = { self.plain_datas.borrow().get(path).unwrap().clone() };
//     // //     let rwops = must(RWops::from_bytes(data.as_slice()));
//     // //     let surface = must(rwops.load());
//     // //     let tc = self.texture_creator.borrow().as_ref().unwrap();
//     // //     let texture = tc.create_texture_from_surface(surface).unwrap();
//     // //     { self.textures.borrow_mut().insert(path.to_owned(), Rc::new(texture)) };
//     // // }
//
// }