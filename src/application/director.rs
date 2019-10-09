use ::application::{ Application, AppDelegate };

pub struct Director {
    application: Application
}

impl Director {

    pub fn new(app_delegate: Box<dyn AppDelegate>) -> Self {
        Self {
            application: Application::new(app_delegate)
        }
    }

    pub fn run_with_scene<T>(scene: T) {

    }

}