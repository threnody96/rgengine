mod resource_manager;
mod plaindata_manager;
mod image_manager;
mod sound_manager;
mod font_manager;
mod text_manager;
mod json_manager;

pub use self::resource_manager::ResourceManager;
pub use self::plaindata_manager::PlaindataManager;
pub use self::image_manager::ImageManager;
pub use self::sound_manager::SoundManager;
pub use self::font_manager::FontManager;
pub use self::text_manager::TextManager;
pub use self::json_manager::JsonManager;
