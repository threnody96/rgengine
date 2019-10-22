extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate image; 
pub extern crate ggez;
extern crate crypto;
extern crate rusqlite;
extern crate base64;
extern crate html5ever;

pub mod director;
pub mod resource;
pub mod util;
pub mod application;
pub mod node;

thread_local! {
    pub static DIRECTOR: ::director::Director = ::director::Director::new();
}
