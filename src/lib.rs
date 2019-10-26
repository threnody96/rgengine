extern crate sdl2;
extern crate time;
extern crate crypto;
extern crate base64;
extern crate rusqlite;

pub mod application;
pub mod director;
pub mod util;
pub mod node;
pub mod resource;

thread_local! {
    static DIRECTOR: ::director::Director = ::director::Director::new();
}
