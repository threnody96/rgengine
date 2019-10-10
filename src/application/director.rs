use ::application::{ Application, AppDelegate };
use ::util::{ build_mode, BuildMode, Point, Size };

pub struct Director {
    application: Application,
    display_stats: bool
}

impl Director {

    pub fn new(app_delegate: Box<dyn AppDelegate>) -> Self {
        Self {
            application: Application::new(app_delegate),
            display_stats: build_mode() == BuildMode::Release
        }
    }

    pub fn get_visible_size(&self) -> Size {
        Size { width: 640.0f32, height: 480.0f32 }
    }

    pub fn get_visible_origin(&self) -> Point {
        Point { x: 0.0f32, y: 0.0f32 }
    }

    pub fn run_with_scene<T>(&self, scene: T) {

    }

}