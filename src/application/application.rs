

pub trait Application {

    fn application_did_finish_launching(&self) -> bool {
        true
    }

    fn init_gl_context_attrs(&self) { }

    fn application_did_enter_background(&self) { }

    fn application_will_enter_foreground(&self) { }

    fn fps(&self) -> u32 { 60 }

    fn title(&self) -> String;

    fn window_width(&self) -> u32 {
        800
    }

    fn window_height(&self) -> u32 {
        600
    }

}