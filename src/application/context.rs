use std::rc::Rc;
use std::collections::HashMap;
use ::util::{ Must };
use ::application::{ Application };
use ::resource::{ ResourceKey };
use sdl2::render::{ Canvas, TextureCreator };
use sdl2::video::{ WindowContext, Window };
use sdl2::{ EventPump };
use sdl2::ttf::{ Sdl2TtfContext };

pub struct Context {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl2TtfContext,
    pub font_datas: HashMap<ResourceKey, Box<[u8]>>
}

impl Context {

    pub fn new(application: Rc<dyn Application>) -> Self {
        let (canvas, event_pump) = Self::build(application);
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            ttf_context: sdl2::ttf::init().must(),
            font_datas: HashMap::new()
        }
    }

    fn build(application: Rc<dyn Application>) -> (Canvas<Window>, EventPump) {
        let sdl_context = sdl2::init().must();
        let video_subsystem = sdl_context.video().must();
        let window_size = application.window_size();
        let window = video_subsystem
            .window(application.title().as_str(), window_size.width(), window_size.height())
            .opengl()
            .position_centered()
            .build()
            .must();
        let gl = Self::find_sdl_gl_driver().must();
        (
            window.into_canvas().index(gl).build().must(),
            sdl_context.event_pump().must()
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

    pub fn get_font_data(&self, resource_key: &ResourceKey) -> &Box<[u8]> {
        self.font_datas.get(resource_key).unwrap()
    }

    pub fn add_font_data(&mut self, resource_key: &ResourceKey, data: Rc<Vec<u8>>) {
        if self.font_datas.get(resource_key).is_none() {
            let d = (&*data).clone();
            self.font_datas.insert(resource_key.clone(), d.into_boxed_slice());
        }
    }

}