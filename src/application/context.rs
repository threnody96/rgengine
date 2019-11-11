use std::rc::Rc;
use std::collections::HashMap;
use ::application::{ Application };
use ::resource::{ ResourceKey };
use ::util::parameter::{ Size };
use sdl2::render::{ Canvas, TextureCreator };
use sdl2::video::{ WindowContext, Window };
use sdl2::{ EventPump };
use sdl2::ttf::{ Sdl2TtfContext };
use sdl2::render::{ Texture, BlendMode };
use sdl2::pixels::{ PixelFormatEnum, Color };
use sdl2::rwops::{ RWops };

pub struct Context<'a> {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl2TtfContext,
    pub static_datas: HashMap<ResourceKey, Box<[u8]>>,
    pub static_rwops: HashMap<ResourceKey, RWops<'a>>
}

impl <'a> Context<'a> {

    pub fn new(application: Rc<dyn Application>) -> Self {
        let (canvas, event_pump) = Self::build(application);
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            ttf_context: sdl2::ttf::init().unwrap(),
            static_datas: HashMap::new(),
            static_rwops: HashMap::new()
        }
    }

    fn build(application: Rc<dyn Application>) -> (Canvas<Window>, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        sdl_context.audio().unwrap();
        sdl2::mixer::init(sdl2::mixer::InitFlag::all()).unwrap();
        sdl2::mixer::open_audio(22050, sdl2::mixer::AUDIO_S16SYS, 2, 4096).unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        // gl_attr.set_multisample_buffers(2);
        // gl_attr.set_multisample_samples(4);
        let window_size = application.window_size();
        let window = video_subsystem
            .window(application.title().as_str(), window_size.width(), window_size.height())
            .opengl()
            .position_centered()
            .build()
            .unwrap();
        let gl = Self::find_sdl_gl_driver().unwrap();
        (
            window.into_canvas().index(gl).build().unwrap(),
            sdl_context.event_pump().unwrap()
        )
    }

    fn find_sdl_gl_driver() -> Result<u32, String> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Ok(index as u32);
            }
        }
        Err("OpenGL の初期化に失敗しました".to_owned())
    }

    pub fn get_static_data(&self, resource_key: &ResourceKey) -> Option<&Box<[u8]>> {
        self.static_datas.get(resource_key)
    }

    pub fn get_static_rwops(&mut self, resource_key: &ResourceKey) -> Option<&RWops<'a>> {
        self.static_rwops.get(resource_key)
    }

    pub fn add_static_rwops(&mut self, resource_key: &ResourceKey, rwops: RWops<'a>) {
        self.static_rwops.insert(resource_key.clone(), rwops);
    }

    pub fn add_static_data(&mut self, resource_key: &ResourceKey, data: Rc<Vec<u8>>) {
        if self.static_datas.get(resource_key).is_none() {
            let d = (&*data).clone();
            self.static_datas.insert(resource_key.clone(), d.into_boxed_slice());
        }
    }

    pub fn create_sub_canvas(&'a mut self, size: Size) -> Texture<'a> {
        let mut texture = self.texture_creator.create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            size.width(),
            size.height()
        ).unwrap();
        self.canvas.with_texture_canvas(&mut texture, |can| {
            can.set_blend_mode(BlendMode::None);
            can.set_draw_color(Color::RGBA(0, 0, 0, 0));
            can.clear();
        }).unwrap();
        texture
    }

}
