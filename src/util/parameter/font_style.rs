use std::ops::{ Deref, DerefMut };
use ::util::{ NoOption };

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct FontStyle {
    style: sdl2::ttf::FontStyle
}

impl FontStyle {

    pub fn normal() -> Self {
        Self::from(sdl2::ttf::FontStyle::NORMAL)
    }

    pub fn bold() -> Self {
        Self::from(sdl2::ttf::FontStyle::BOLD)
    }

    pub fn italic() -> Self {
        Self::from(sdl2::ttf::FontStyle::ITALIC)
    }

    pub fn underline() -> Self {
        Self::from(sdl2::ttf::FontStyle::UNDERLINE)
    }

    pub fn strikethrough() -> Self {
        Self::from(sdl2::ttf::FontStyle::STRIKETHROUGH)
    }

}


impl Deref for FontStyle {

    type Target = sdl2::ttf::FontStyle;

    fn deref(&self) -> &Self::Target {
        &self.style
    }

}

impl Into<sdl2::ttf::FontStyle> for FontStyle {

    fn into(self) -> sdl2::ttf::FontStyle {
        self.style
    }

}

impl DerefMut for FontStyle {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }

}

impl From<sdl2::ttf::FontStyle> for FontStyle {

    fn from(f: sdl2::ttf::FontStyle) -> Self {
        Self { style: f }
    }

}

impl From<&sdl2::ttf::FontStyle> for FontStyle {

    fn from(f: &sdl2::ttf::FontStyle) -> Self {
        Self { style: f.clone() }
    }

}

impl From<&FontStyle> for FontStyle {

    fn from(f: &FontStyle) -> FontStyle {
        f.clone()
    }

}

impl From<NoOption> for FontStyle {

    fn from(f: NoOption) -> FontStyle {
        Self::normal()
    }

}
