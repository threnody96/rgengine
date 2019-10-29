use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::collections::HashMap;
use ::application::{ Application };
use ::node::{ SceneLike, LabelOption };
use ::util::{ canvas, FpsManager, Size, Must };
use sdl2::{ EventPump };
use sdl2::render::{ Canvas, TextureCreator };
use sdl2::video::{ Window, WindowContext};
use sdl2::event::{ Event };
use sdl2::keyboard::{ Keycode };
use uuid::Uuid;
use rand::{ Rng };
use rand::rngs::{ ThreadRng };
use rand::distributions::{ Standard, Distribution };

pub struct ApplicationDirector {
    scene: Option<Rc<dyn SceneLike>>,
    application: Option<Rc<dyn Application>>,
    default_label_option: Option<LabelOption>,
    id_cache: HashMap<String, bool>,
    current_fps: usize,
    rand: ThreadRng,
    continuing: bool
}

impl ApplicationDirector {

    pub fn new() -> Self {
        Self {
            scene: None,
            application: None,
            default_label_option: None,
            id_cache: HashMap::new(),
            current_fps: 0,
            rand: rand::thread_rng(),
            continuing: true
        }
    }

    pub fn window_size(&self) -> Size {
        self.application().window_size()
    }

    pub fn rand<T>(&mut self) -> T where Standard: Distribution<T> {
        self.rand.gen()
    }

    pub fn is_continuing(&self) -> bool {
        self.continuing
    }

    pub fn set_continuing(&mut self, continuing: bool) {
        self.continuing = continuing;
    }

    pub fn default_label_option(&self) -> Option<LabelOption> {
        self.default_label_option.clone()
    }

    pub fn set_default_label_option(&mut self, option: &LabelOption) {
        self.default_label_option = Some(option.clone());
    }

    pub fn set_scene(&mut self, scene: Rc<dyn SceneLike>) {
        self.scene = Some(scene);
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.scene.clone().unwrap()
    }

    pub fn set_application(&mut self, application: Rc<dyn Application>) {
        self.application = Some(application);
    }

    pub fn application(&self) -> Rc<dyn Application> {
        self.application.clone().ok_or("application not found").must()
    }

    pub fn set_current_fps(&mut self, current_fps: usize) {
        self.current_fps = current_fps;
    }

    pub fn current_fps(&self) -> usize {
        self.current_fps
    }

    pub fn generate_id(&mut self) -> String {
        let mut id = "".to_owned();
        loop {
            id = Uuid::new_v4().to_string();
            if self.id_cache.get(&id).is_none() { break; }
        }
        self.id_cache.insert(id.clone(), true);
        id
    }

}
