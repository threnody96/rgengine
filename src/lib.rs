extern crate sdl2;

pub mod application;
pub mod director;
pub mod util;
pub mod node;

thread_local! {
    static DIRECTOR: ::director::Director = ::director::Director::new();
}
