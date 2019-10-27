extern crate sdl2;
extern crate time;
extern crate crypto;
extern crate base64;
extern crate rusqlite;
extern crate uuid;

pub mod application;
pub mod director;
pub mod util;
pub mod node;
pub mod resource;

use ::director::Director;
use ::director::RenderDirector;

use std::cell::RefCell;

thread_local! {
    pub static DIRECTOR: Director = Director::new();

    pub static RENDER: RefCell<RenderDirector<'static>> = RefCell::new(RenderDirector::new());
}
