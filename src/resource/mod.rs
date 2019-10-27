mod file_storage;
mod encrypt_storage;
mod texture;
mod font;

#[cfg(build_mode = "release")]
pub type Storage = self::encrypt_storage::EncryptStorage;

#[cfg(not(build_mode = "release"))]
pub type Storage = self::file_storage::FileStorage;

// pub use self::resource_loader::*;
pub use self::texture::*;
pub use self::font::*;
