use std::rc::Rc;
use ::util::{ must };
use sdl2::ttf::{ Sdl2TtfContext, Font };
use sdl2::rwops::{ RWops };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RFont {
    key: String
}

impl RFont {

    pub fn new(key: &str) -> Self {
        Self { key: key.to_owned() }
    }

}

pub struct FontFactory<'a> {
    plain_data: Rc<Vec<u8>>,
    point: u16,
    font: Option<Font<'a, 'a>>
}

impl <'a> FontFactory<'a> {

    pub fn new(plain_data: Rc<Vec<u8>>, point: u16) -> Self {
        Self {
            plain_data: plain_data,
            point: point,
            font: None
        }
    }

    pub fn font(&'a mut self, ttf_context: &'a Sdl2TtfContext) -> &'a Font<'a, 'a> {
        if self.font.is_some() { return self.font.as_ref().unwrap(); }
        let rwops = must(RWops::from_bytes(self.plain_data.as_slice()));
        let font = must(ttf_context.load_font_from_rwops(rwops, self.point));
        self.font = Some(font);
        self.font.as_ref().unwrap()
    }

}

