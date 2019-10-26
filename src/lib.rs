extern crate sdl2;
extern crate time;

pub mod application;
pub mod director;
pub mod util;
pub mod node;

thread_local! {
    static DIRECTOR: ::director::Director = ::director::Director::new();
}
