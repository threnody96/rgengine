use ::application::{ AppDelegate, Director };

pub struct Application<T> where T: AppDelegate {
    delegate: T,
    director: Director
}

impl <T> Application<T> where T: AppDelegate {

    pub fn new(delegate: T) -> Self {
        Self {
            delegate: delegate,
            director: Director::new()
        }
    }

    pub fn run(&self) -> Result<(), String> {
        Ok(())
    }

}