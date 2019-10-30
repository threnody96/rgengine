mod file_storage;
mod encrypt_storage;
mod texture;
mod font;
mod resource_key;
mod resource_type;

#[cfg(build_mode = "release")]
pub type Storage = self::encrypt_storage::EncryptStorage;

#[cfg(not(build_mode = "release"))]
pub type Storage = self::file_storage::FileStorage;

pub use self::texture::*;
pub use self::font::*;
pub use self::resource_key::*;
pub use self::resource_type::*;
