use ::application::{ AppDelegate };

pub struct Application {
    delegate: Box<dyn AppDelegate>,
}

impl Application {

    pub fn new(delegate: Box<dyn AppDelegate>) -> Self {
        Self {
            delegate: delegate,
        }
    }

}