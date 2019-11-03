extern crate sdl2;
extern crate time;
extern crate crypto;
extern crate base64;
extern crate rusqlite;
extern crate uuid;
extern crate serde_json;
extern crate rand;
extern crate backtrace;
extern crate html5ever;
extern crate chrono;

pub mod application;
pub mod director;
pub mod util;
pub mod node;
pub mod resource;

use ::director::Director;
use ::application::Context;

thread_local! {
    pub static DIRECTOR: Director<'static> = Director::new();
}

pub(crate) static mut CONTEXT: Option<Context> = None;

pub static NoOption: ::util::NoOption = ::util::NoOption {};
