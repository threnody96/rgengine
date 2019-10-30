use std::rc::Rc;
use ::resource::{ ResourceKey };
use ::util::{ Must, context };
use sdl2::ttf::{ Sdl2TtfContext, Font, FontStyle };
use sdl2::rwops::{ RWops };

#[derive(Clone)]
pub struct RFont {
    key: ResourceKey
}

impl RFont {

    pub fn new(key: &ResourceKey) -> Self {
        Self { key: key.clone() }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

}

pub struct FontFactory<'a> {
    point: u16,
    style: FontStyle,
    font: Rc<Font<'a, 'a>>
}

pub struct FontWrapper {

}

impl <'a> FontFactory<'a> {

    pub fn new(plain_data: &'a [u8], point: u16, style: FontStyle) -> Self {
        let rwops = RWops::from_bytes(plain_data).must();
        let mut font = context(|c| c.ttf_context.load_font_from_rwops(rwops, point)).must();
        font.set_style(style);
        Self {
            point: point,
            style: style,
            font: Rc::new(font)
        }
    }

    pub fn font(&self) -> Rc<Font<'a, 'a>> {
        self.font.clone()
    }

}

