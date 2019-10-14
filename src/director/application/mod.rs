use std::rc::Rc;
use std::cell::RefCell;
use ::application::{ Application, AppDelegate };
use ::node::{ Node, Scene };
use ::util::{ BuildMode, build_mode, Size, Point };

pub struct ApplicationDerector {
    application: Application,
    scene: RefCell<Option<Rc<dyn Scene>>>,
    display_stats: bool
}

impl ApplicationDerector {

    pub fn new() -> Self {
        Self {
            application: Application::new(),
            display_stats: build_mode() == BuildMode::Development,
            scene: RefCell::new(None)
        }
    }

    pub fn set_scene(&self, scene: Rc<dyn Scene>) {
        self.scene.borrow().as_ref().map(|s| {
            if s.id() != scene.id() { s.destroy(); }
        });
        self.scene.replace(Some(scene));
    }

    pub fn set_visible_size(&self, size: Size) {
        self.application.set_visible_size(size);
    }

}
