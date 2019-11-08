use ::util::parameter::{ FontStyle };

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ResourceType {
    PlainData,
    String,
    Json,
    Texture,
    Font(u16, FontStyle),
    Music,
    SE,
    RenderCache
}
