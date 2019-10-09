pub trait AppDelegate {

    fn application_did_finish_launching(&self) -> bool;

    fn init_gl_context_attrs(&self) { }

    fn application_did_enter_background(&self) { }

    fn application_will_enter_foreground(&self) { }

}

