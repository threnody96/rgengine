use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use ::application::{ Application };
use ::node::{ SceneLike };
use ::util::{ must, canvas };
use sdl2::{ EventPump };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };
use sdl2::event::{ Event };
use sdl2::keyboard::{ Keycode };

pub struct ApplicationDirector {
    scene: RefCell<Option<Rc<dyn SceneLike>>>,
    application: RefCell<Option<Rc<dyn Application>>>
}

impl ApplicationDirector {

    pub fn new() -> Self {
        Self {
            scene: RefCell::new(None),
            application: RefCell::new(None)
        }
    }

    pub fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        self.scene.replace(Some(scene));
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.scene.borrow().clone().unwrap()
    }

    pub fn set_application(&self, application: Rc<dyn Application>) {
        self.application.replace(Some(application));
    }

    pub fn application(&self) -> Rc<dyn Application> {
        let application = self.application.borrow();
        must(application.clone().ok_or("application not found"))
    }

    pub fn build(&self) -> (EventPump, Canvas<Window>) {
        let sdl_context = must(sdl2::init());
        let video_subsystem = must(sdl_context.video());
        let title = self.application().title();
        let window = must(video_subsystem.window(&title, 800, 600)
            .opengl()
            .position_centered()
            .build());
        let gl = must(self.find_sdl_gl_driver());
        (
            must(sdl_context.event_pump()),
            must(window.into_canvas().index(gl).build())
        )
    }

    fn find_sdl_gl_driver(&self) -> Result<u32, String> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Ok(index as u32);
            }
        }
        Err("OpenGL の初期化に失敗しました".to_owned())
    }

    pub fn run(&self, event_pump: &mut EventPump) {
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            canvas(|c| c.clear());
            let prev_scene = self.get_scene();
            prev_scene.update();
            let next_scene = self.get_scene();
            if prev_scene.id() == next_scene.id() {
                next_scene.render();
                canvas(|c| c.present());
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.application().fps()));
            }
        }
    }

}