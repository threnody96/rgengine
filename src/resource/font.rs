use std::rc::Rc;
use ::util::{ must };
use sdl2::ttf::{ Sdl2TtfContext, Font, FontStyle };
use sdl2::rwops::{ RWops };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RFont {
    key: String
}

impl RFont {

    pub fn new(key: &str) -> Self {
        Self { key: key.to_owned() }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

}

pub struct FontFactory<'a> {
    plain_data: Rc<Vec<u8>>,
    point: u16,
    style: FontStyle,
    font: Option<Font<'a, 'a>>
}

impl <'a> FontFactory<'a> {

    pub fn new(plain_data: Rc<Vec<u8>>, point: u16, style: FontStyle) -> Self {
        Self {
            plain_data: plain_data,
            point: point,
            style: style,
            font: None
        }
    }

    pub fn font(&'a self) -> &'a Font<'a, 'a> {
        self.font.as_ref().unwrap()
    }

    pub fn generate_font(&'a mut self, ttf_context: &'a Sdl2TtfContext) {
        if self.font.is_some() { return; }
        let rwops = must(RWops::from_bytes(self.plain_data.as_slice()));
        let mut font = must(ttf_context.load_font_from_rwops(rwops, self.point));
        font.set_style(self.style);
        self.font = Some(font);
    }

}

