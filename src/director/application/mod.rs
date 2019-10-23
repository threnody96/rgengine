use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ::application::{ AppDelegate, ResolutionSize, ResolutionPolicy, Game };
use ::node::{ Node, SceneLike, NodeLike, LabelTextOption };
use ::util::{ BuildMode, build_mode, Size, Point, run };
use ggez::{ Context, ContextBuilder, event::EventsLoop };
use ggez::graphics::{ Scale, Color };

pub struct ApplicationDerector {
    scene: RefCell<Option<Rc<dyn SceneLike>>>,
    delegate: RefCell<Option<Rc<dyn AppDelegate>>>,
    resolution_size: RefCell<Option<ResolutionSize>>,
    visible_size: RefCell<Option<Size>>,
    font_size: RefCell<HashMap<String, Scale>>,
    font: RefCell<HashMap<String, String>>,
    color: RefCell<HashMap<String, Color>>,
    display_stats: RefCell<bool>
}

impl ApplicationDerector {

    pub fn new() -> Self {
        Self {
            scene: RefCell::new(None),
            delegate: RefCell::new(None),
            visible_size: RefCell::new(None),
            resolution_size: RefCell::new(None),
            font_size: RefCell::new(HashMap::new()),
            font: RefCell::new(HashMap::new()),
            color: RefCell::new(HashMap::new()),
            display_stats: RefCell::new(build_mode() == BuildMode::Development),
        }
    }

    pub fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        self.scene.replace(Some(scene));
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.scene.borrow().clone().unwrap()
    }

    pub fn set_delegate(&self, delegate: Rc<dyn AppDelegate>) {
        self.delegate.replace(Some(delegate));
    }

    pub fn add_font_size(&self, name: String, scale: f32) {
        let mut font_size = self.font_size.borrow_mut();
        font_size.insert(name, Scale::uniform(scale));
    }

    pub fn get_font_size(&self, name: String) -> Option<Scale> {
        let font_size = self.font_size.borrow();
        font_size.get(&name).cloned()
    }

    pub fn add_font(&self, name: String, path: String) {
        let mut font = self.font.borrow_mut();
        font.insert(name, path);
    }

    pub fn get_font(&self, name: String) -> Option<String> {
        let font = self.font.borrow();
        font.get(&name).cloned()
    }

    pub fn add_color(&self, name: String, color: Color) {
        let mut c = self.color.borrow_mut();
        c.insert(name, color);
    }

    pub fn get_color(&self, name: String) -> Option<Color> {
        let color = self.color.borrow();
        color.get(&name).cloned()
    }

    pub fn get_visible_size(&self) -> Size {
        let visible_size = self.visible_size.borrow();
        if visible_size.is_none() { panic!("ゲームが実行されていません"); }
        visible_size.clone().unwrap()
    }

    pub fn set_visible_size(&self, size: Size) {
        self.visible_size.replace(Some(size));
    }

    pub fn get_visible_origin(&self) -> Point {
        Point { x: 0.0f32, y: 0.0f32 }
    }

    pub fn get_resolution_size(&self) -> ResolutionSize {
        let resolution_size = self.resolution_size.borrow();
        if resolution_size.is_none() { panic!("ゲームが実行されていません"); }
        resolution_size.clone().unwrap()
    }

    pub fn get_default_label_option(&self) -> LabelTextOption {
        let delegate = self.delegate.borrow();
        match delegate.as_ref() {
            None => {
                LabelTextOption::default()
            },
            Some(d) => {
                d.application_setup().default_label_option.clone()
            }
        }
    }

    pub fn set_resolution_size(&self, size: Size, policy: ResolutionPolicy) {
        self.resolution_size.replace(Some(ResolutionSize {
            size: size,
            policy: policy
        }));
    }

    pub fn set_display_stats(&self, display_stats: bool) {
        self.display_stats.replace(display_stats);
    }

    pub fn get_display_stats(&self) -> bool {
        self.display_stats.borrow().clone()
    }

    pub fn run(&self, event_loop: &mut EventsLoop) {
        let delegate = self.delegate.borrow().clone().unwrap();
        let mut game = Game::new(delegate);
        match run(event_loop, &mut game) {
            Ok(_) => { },
            Err(e) => { panic!(format!("初期化に失敗しました: {}", e)); }
        }
    }

    pub fn init(&self, delegate: Rc<dyn AppDelegate>, scene: Rc<dyn SceneLike>) -> (Context, EventsLoop) {
        let size = delegate.application_setup().generate_window_size();
        self.set_scene(scene);
        self.set_delegate(delegate);
        self.set_visible_size(size.clone());
        self.set_resolution_size(size, ResolutionPolicy::ShowAll);
        self.build()
    }

    fn build(&self) -> (Context, EventsLoop) {
        let delegate = self.delegate.borrow().clone().unwrap();
        let setup = delegate.application_setup();
        ContextBuilder::new("game_id", &delegate.author())
            .window_mode(delegate.window_mode().unwrap_or(setup.generate_window_mode()))
            .window_setup(delegate.window_setup().unwrap_or(setup.generate_window_setup()))
            .build()
            .expect("aieee, could not create ggez context!")
    }

}
