use std::rc::Rc;
use ::application::{ Application, AppDelegate };
use ::node::{ Node, Scene, BlankScene };
use ::util::{ BuildMode, build_mode, Size, Point };

pub struct ApplicationDerector {
    application: Application,
    scene: Rc<Scene>,
    display_stats: bool
}

impl ApplicationDerector {

    pub fn new(app_delegate: Box<dyn AppDelegate>) -> Self {
        Self {
            application: Application::new(app_delegate),
            display_stats: build_mode() == BuildMode::Release,
            scene: Node::create(|| { BlankScene {} })
        }
    }

    pub fn get_visible_size(&self) -> Size {
        Size { width: 640.0f32, height: 480.0f32 }
    }

    pub fn get_visible_origin(&self) -> Point {
        Point { x: 0.0f32, y: 0.0f32 }
    }

    pub fn replace_scene(&mut self, scene: Rc<Scene>) {
        self.scene.destroy();
        self.scene = scene;
    }

    pub fn run_with_scene<T>(&self, scene: T) {

    }

}
