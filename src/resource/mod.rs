mod file_storage;
mod packed_storage;
mod texture;
mod font;
mod se;
mod resource_key;
mod resource_type;

pub use self::texture::*;
pub use self::font::*;
pub use self::se::*;
pub use self::resource_key::*;
pub use self::resource_type::*;
pub use self::file_storage::*;
pub use self::packed_storage::*;

#[cfg(build_mode = "release")]
pub type Storage = PackedStorage;

#[cfg(not(build_mode = "release"))]
pub type Storage = FileStorage;

