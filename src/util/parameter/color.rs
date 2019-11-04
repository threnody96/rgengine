use std::ops::Deref;
use sdl2::pixels::PixelFormat;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color {
    color: sdl2::pixels::Color
}

impl Deref for Color {

    type Target = sdl2::pixels::Color;

    fn deref(&self) -> &Self::Target {
        &self.color
    }

}

impl Color {

    pub fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: sdl2::pixels::Color::RGB(r, g, b)
        }
    }

    pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color: sdl2::pixels::Color::RGBA(r, g, b, a)
        }
    }

    pub fn from_u32(format: &PixelFormat, pixel: u32) -> Self {
        Self {
            color: sdl2::pixels::Color::from_u32(format, pixel)
        }
    }

}

impl From<sdl2::pixels::Color> for Color {

    fn from(f: sdl2::pixels::Color) -> Self {
        Self { color: f }
    }

}

impl From<&sdl2::pixels::Color> for Color {

    fn from(f: &sdl2::pixels::Color) -> Self {
        Self { color: f.clone() }
    }

}

impl From<(u8, u8, u8)> for Color {

    fn from(f: (u8, u8, u8)) -> Self {
        Self::RGB(f.0, f.1, f.2)
    }

}

impl From<(u8, u8, u8, u8)> for Color {

    fn from(f: (u8, u8, u8, u8)) -> Self {
        Self::RGBA(f.0, f.1, f.2, f.3)
    }

}
