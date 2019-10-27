use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ::resource::{ RTexture, FontFactory, RFont, Storage };
use ::util::{ must, director };
use sdl2::render::{ Canvas, Texture, TextureCreator };
use sdl2::video::{ WindowContext, Window };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };
use sdl2::{ EventPump };
use sdl2::ttf::{ Sdl2TtfContext, Font };
use serde_json::Value;
use uuid::Uuid;

pub struct RenderDirector<'a> {
    canvas: Option<Canvas<Window>>,
    texture_creator: Option<TextureCreator<WindowContext>>,
    ttf_context: Option<Sdl2TtfContext>,
    storage: Storage,
    plain_datas: HashMap<String, Rc<Vec<u8>>>,
    strings: HashMap<String, Rc<String>>,
    jsons: HashMap<String, Rc<Value>>,
    textures: HashMap<String, Texture<'a>>,
    fonts: HashMap<String, FontFactory<'a>>
}

impl <'a> RenderDirector<'a> {

    pub fn new() -> Self {
        Self {
            canvas: None,
            texture_creator: None,
            ttf_context: None,
            storage: Storage::new(),
            plain_datas: HashMap::new(),
            strings: HashMap::new(),
            jsons: HashMap::new(),
            textures: HashMap::new(),
            fonts: HashMap::new()
        }
    }

    fn find_sdl_gl_driver(&self) -> Result<u32, String> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Ok(index as u32);
            }
        }
        Err("OpenGL の初期化に失敗しました".to_owned())
    }

    pub fn with_canvas<T, R>(&mut self, callback: T) -> R where T: FnOnce(&mut Canvas<Window>) -> R {
        callback(self.canvas.as_mut().unwrap())
    }

    pub fn build(&mut self) -> EventPump {
        let sdl_context = must(sdl2::init());
        let video_subsystem = must(sdl_context.video());
        let title = director(|d| d.title());
        let window = must(video_subsystem.window(&title, 800, 600)
            .opengl()
            .position_centered()
            .build());
        let gl = must(self.find_sdl_gl_driver());
        let canvas = must(window.into_canvas().index(gl).build());
        self.texture_creator = Some(canvas.texture_creator());
        self.canvas = Some(canvas);
        self.ttf_context = Some(must(sdl2::ttf::init()));
        must(sdl_context.event_pump())
    }

    pub fn load_plain_data(&mut self, path: &str) -> Rc<Vec<u8>> {
        if let Some(current) = self.plain_datas.get(path) {
            current.clone()
        } else {
            let data = Rc::new(must(self.storage.load(path)));
            self.plain_datas.insert(path.to_owned(), data.clone());
            data
        }
    }

    pub fn load_string(&mut self, path: &str) -> Rc<String> {
        if let Some(current) = self.strings.get(path) {
            current.clone()
        } else {
            let data = self.load_plain_data(path);
            let s = Rc::new(must(String::from_utf8(data.as_ref().clone())));
            self.strings.insert(path.to_owned(), s.clone());
            s
        }
    }

    pub fn load_json(&mut self, path: &str) -> Rc<Value> {
        if let Some(current) = self.jsons.get(path) {
            current.clone()
        } else {
            let data = self.load_string(path);
            let json: Value = must(serde_json::from_str(data.as_str()));
            let j = Rc::new(json);
            self.jsons.insert(path.to_owned(), j.clone());
            j
        }
    }

    pub fn load_texture(&'a mut self, path: &str) -> String {
        let data = self.load_plain_data(path);
        let rwops = must(RWops::from_bytes(data.as_slice()));
        let surface = must(rwops.load());
        let texture_creator = self.texture_creator.as_ref().unwrap();
        let texture = must(texture_creator.create_texture_from_surface(surface));
        let id = director(|d| d.generate_id());
        self.textures.insert(id.clone(), texture);
        id
    }

    pub fn load_font(&'a mut self, path: &str, point: u16) -> String {
        let data = self.load_plain_data(path);
        let font = FontFactory::new(data, point);
        let id = director(|d| d.generate_id());
        self.fonts.insert(id.clone(), font);
        id
    }

}