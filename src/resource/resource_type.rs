use sdl2::ttf::{ FontStyle };

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ResourceType {
    PlainData,
    String,
    Json,
    Texture,
    Font(u16, FontStyle),
    RenderCache
}
