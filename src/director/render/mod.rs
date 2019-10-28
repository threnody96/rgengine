use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ::node::{ NodeLike };
use ::resource::{ RTexture, FontFactory, RFont, Storage };
use ::application::{ Application, ResolutionPolicy };
use ::util::{ must, director, Size, Rect, Point };
use sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use sdl2::video::{ WindowContext, Window };
use sdl2::rwops::{ RWops };
use sdl2::image::{ ImageRWops };
use sdl2::{ EventPump };
use sdl2::ttf::{ Sdl2TtfContext, Font, FontStyle };
use sdl2::pixels::{ PixelFormatEnum, Color };
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone)]
pub enum RenderOperation {
    Image(Point, RTexture),
    Label(Point, String, RFont, Color)
}

pub struct RenderDirector<'a> {
    canvas: Option<Canvas<Window>>,
    canvas_size: Size,
    inner_canvas: Option<Texture<'a>>,
    resolution_size: Size,
    resolution_policy: ResolutionPolicy,
    render_canvas_dest: Option<Rect>,
    render_operations: Vec<RenderOperation>,
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
            canvas_size: Size { width: 0, height: 0 },
            inner_canvas: None,
            resolution_size: Size { width: 0, height: 0 },
            resolution_policy: ResolutionPolicy::ExactFit,
            render_canvas_dest: None,
            render_operations: Vec::new(),
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

    pub fn build(&'a mut self, application: Rc<dyn Application>) -> EventPump {
        let sdl_context = must(sdl2::init());
        let video_subsystem = must(sdl_context.video());
        let canvas_size = application.window_size();
        let window = must(video_subsystem
            .window(application.title().as_str(), canvas_size.width, canvas_size.height)
            .opengl()
            .position_centered()
            .build());
        let gl = must(self.find_sdl_gl_driver());
        let canvas = must(window.into_canvas().index(gl).build());
        self.texture_creator = Some(canvas.texture_creator());
        self.canvas_size = canvas_size;
        self.resolution_size = application.resolution_size();
        self.resolution_policy = application.resolution_policy();
        self.render_canvas_dest = self.generate_render_canvas_dest();
        self.canvas = Some(canvas);
        let texture_creator = self.texture_creator.as_ref().unwrap();
        self.inner_canvas = Some(must(texture_creator.create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            self.resolution_size.width,
            self.resolution_size.height
        )));
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

    pub fn load_font(&'a mut self, path: &str, point: u16, style: FontStyle) -> String {
        let data = self.load_plain_data(path);
        let id = director(|d| d.generate_id());
        self.fonts.insert(id.clone(), FontFactory::new(data, point, style));
        let font = self.fonts.get_mut(&id).unwrap();
        font.generate_font(self.ttf_context.as_ref().unwrap());
        id
    }

    pub fn render_texture(&mut self, point: Point, texture: &RTexture) {
        self.render_operations.push(RenderOperation::Image(point, texture.clone()));
    }

    pub fn render_label(&mut self, point: Point, text: &str, font: &RFont, color: &Color) {
        self.render_operations.push(RenderOperation::Label(point, text.to_owned(), font.clone(), color.clone()));
    }

    pub fn update_resolution_size(&'a mut self, resolution_size: Size, resolution_policy: ResolutionPolicy) {
        if self.resolution_size != resolution_size || self.resolution_policy != resolution_policy {
            self.resolution_size = resolution_size;
            self.resolution_policy = resolution_policy;
            let texture_creator = self.texture_creator.as_ref().unwrap();
            self.inner_canvas = Some(must(texture_creator.create_texture_target(
                Some(PixelFormatEnum::RGBA8888),
                self.resolution_size.width,
                self.resolution_size.height
            )));
            self.render_canvas_dest = self.generate_render_canvas_dest();
        }
    }

    pub fn render_inner_canvas(&'a mut self) {
        let canvas = self.canvas.as_mut().unwrap();
        let inner_canvas = self.inner_canvas.as_mut().unwrap();
        let operations = &self.render_operations;
        let textures = &self.textures;
        let fonts = &self.fonts;
        let texture_creator = self.texture_creator.as_ref().unwrap();
        must(canvas.with_texture_canvas(inner_canvas, |c| {
            c.set_blend_mode(BlendMode::Blend);
            c.clear();
            for operation in operations {
                match operation {
                    RenderOperation::Image(point, texture) => {
                        if let Some(t) = textures.get(texture.key().as_str()) {
                            let query = t.query();
                            must(c.copy(t, None, Some(Rect::new(point.x(), point.y(), query.width, query.height))));
                        }
                    },
                    RenderOperation::Label(point, text, font, color) => {
                        if let Some(f) = fonts.get(font.key().as_str()) {
                            let font = f.font();
                            let surface = must(font.render(text.as_str()).blended(*color));
                            let texture = must(texture_creator.create_texture_from_surface(surface));
                            let query = texture.query();
                            must(c.copy(&texture, None, Some(Rect::new(point.x(), point.y(), query.width, query.height))));
                        }
                    }
                }
            }
        }));
        self.render_operations = Vec::new();
    }

    pub fn render_canvas(&'a mut self) {
        let canvas = self.canvas.as_mut().unwrap();
        let inner_canvas = self.inner_canvas.as_ref().unwrap();
        canvas.clear();
        canvas.copy(inner_canvas, None, self.render_canvas_dest.clone());
        canvas.present();
    }

    fn generate_render_canvas_dest(&self) -> Option<Rect> {
        if self.resolution_policy == ResolutionPolicy::ExactFit { return None; }
        let rsize = &self.resolution_size;
        let canvas_size = &self.canvas_size;
        let per_size = (canvas_size.width as f32 / rsize.width as f32, canvas_size.height as f32 / rsize.height as f32);
        let use_per_size = self.choice_per_size_from_policy(per_size.0, per_size.1, self.resolution_policy);
        let dest_size = ((rsize.width as f32 * use_per_size).round() as i32, (rsize.height as f32 * use_per_size).round() as i32);
        let dest_center = (dest_size.0 / 2, dest_size.1 / 2);
        let real_center = (canvas_size.width as i32 / 2, canvas_size.height as i32 / 2);
        let render_to = (real_center.0 - dest_center.0, real_center.1 - dest_center.1);
        Some(Rect::new(render_to.0, render_to.1, dest_size.0 as u32, dest_size.1 as u32))
    }

    fn choice_per_size_from_policy(&self, per_width: f32, per_height: f32, policy: ResolutionPolicy) -> f32 {
        match policy {
            ResolutionPolicy::ShowAll => { if per_width > per_height { per_height } else { per_width } },
            ResolutionPolicy::NoBorder => { if per_width > per_height { per_width } else { per_height } },
            ResolutionPolicy::FixedWidth => { per_width },
            ResolutionPolicy::FixedHeight => { per_height },
            ResolutionPolicy::ExactFit => { panic!("不到達コード"); },
        }
    }

}
