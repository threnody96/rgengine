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
        Self::from(f.clone())
    }

}

impl From<&Color> for Color {

    fn from(f: &Color) -> Self {
        f.clone()
    }

}

impl <A, B, C> From<(A, B, C)> for Color
where
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>
{

    fn from(f: (A, B, C)) -> Self {
        Self::RGB(f.0.into(), f.1.into(), f.2.into())
    }

}

impl <A, B, C, D> From<(A, B, C, D)> for Color
where
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>,
    D: Into<u8>
{

    fn from(f: (A, B, C, D)) -> Self {
        Self::RGBA(f.0.into(), f.1.into(), f.2.into(), f.3.into())
    }

}
