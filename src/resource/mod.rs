mod file_resource;
mod encrypt_resource;

#[cfg(build_mode = "release")]
pub type Resource = self::encrypt_resource::EncryptStorage;

#[cfg(not(build_mode = "release"))]
pub type Resource = self::file_resource::FileStorage;

