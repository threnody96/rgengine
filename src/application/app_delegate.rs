use ::util::{ Size };
use ::application::application_setup::ApplicationSetup;
use ggez::conf::{ WindowSetup, WindowMode };

pub trait AppDelegate {

    fn application_did_finish_launching(&self) -> bool;

    fn init_gl_context_attrs(&self) { }

    fn application_did_enter_background(&self) { }

    fn application_will_enter_foreground(&self) { }

    fn author(&self) -> String;

    fn fps(&self) -> u32 { 60 }

    fn window_setup(&self) -> Option<WindowSetup> {
        None
    }

    fn window_mode(&self) -> Option<WindowMode> {
        None
    }

    fn application_setup(&self) -> ApplicationSetup {
        ApplicationSetup::default()
    }

}

