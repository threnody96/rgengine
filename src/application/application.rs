use std::rc::Rc;
use ::node::{ SceneLike };
use ::application::{ ResolutionPolicy };
use util::{ Size };

pub trait Application {

    fn application_did_finish_launching(&self) -> Rc<dyn SceneLike>;

    fn init_gl_context_attrs(&self) { }

    fn application_did_enter_background(&self) { }

    fn application_will_enter_foreground(&self) { }

    fn fps(&self) -> u32 { 60 }

    fn title(&self) -> String;

    fn window_size(&self) -> Size {
        Size::new(800, 600)
    }

    fn resolution_size(&self) -> Size {
        self.window_size()
    }

    fn resolution_policy(&self) -> ResolutionPolicy {
        ResolutionPolicy::ExactFit
    }

}