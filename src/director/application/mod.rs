use std::rc::Rc;
use std::cell::RefCell;
use ::application::{ AppDelegate, ResolutionSize, ResolutionPolicy, Game };
use ::node::{ Node, Scene };
use ::util::{ BuildMode, build_mode, Size, Point };
use ggez::{ Context, ContextBuilder, event::run, event::EventsLoop };

pub struct ApplicationDerector {
    scene: RefCell<Option<Rc<dyn Scene>>>,
    delegate: RefCell<Option<Rc<dyn AppDelegate>>>,
    resolution_size: RefCell<Option<ResolutionSize>>,
    visible_size: RefCell<Option<Size>>,
    display_stats: bool
}

impl ApplicationDerector {

    pub fn new() -> Self {
        Self {
            scene: RefCell::new(None),
            delegate: RefCell::new(None),
            visible_size: RefCell::new(None),
            resolution_size: RefCell::new(None),
            display_stats: build_mode() == BuildMode::Development,
        }
    }

    pub fn run_with_scene(&self, delegate: Rc<dyn AppDelegate>, scene: Rc<dyn Scene>) {
        self.set_scene(scene);
        self.set_delegate(delegate.clone());
        self.run();
    }

    pub fn set_scene(&self, scene: Rc<dyn Scene>) {
        self.scene.replace(Some(scene));
    }

    pub fn set_delegate(&self, delegate: Rc<dyn AppDelegate>) {
        self.delegate.replace(Some(delegate));
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

    pub fn set_resolution_size(&self, size: Size, policy: ResolutionPolicy) {
        self.resolution_size.replace(Some(ResolutionSize {
            size: size,
            policy: policy
        }));
    }

    fn run(&self) {
        let delegate = self.delegate.borrow().clone().unwrap();
        let size = Size {
            width: delegate.window_mode().width,
            height: delegate.window_mode().height
        };
        self.set_visible_size(size.clone());
        self.set_resolution_size(size, ResolutionPolicy::ShowAll);
        let (mut ctx, mut event_loop) = self.build();
        let mut game = Game::new(&mut ctx, delegate);
        match run(&mut ctx, &mut event_loop, &mut game) {
            Ok(_) => { },
            Err(e) => { panic!(format!("初期化に失敗しました: {}", e)); }
        }
    }

    fn build(&self) -> (Context, EventsLoop) {
        let delegate = self.delegate.borrow().clone().unwrap();
        ContextBuilder::new("game_id", &delegate.author())
            .window_mode(delegate.window_mode())
            .window_setup(delegate.window_setup())
            .build()
            .expect("aieee, could not create ggez context!")
    }

}
