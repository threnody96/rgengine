use std::ops::{ Add, Sub, Mul, Div, Deref };

#[derive(Clone, Copy)]
pub struct Scale {
    scale: f64
}

impl Scale {

    pub fn new(scale: f64) -> Self {
        let s = if scale < 0.0 { 0.0 } else { scale };
        Self {
            scale: s
        }
    }

}

impl Default for Scale {

    fn default() -> Self {
        Self::new(1.0)
    }

}

impl From<Scale> for f64 {

    fn from(f: Scale) -> f64 {
        f.scale
    }

}

impl From<f64> for Scale {

    fn from(f: f64) -> Self {
        Self::new(f)
    }

}

impl Deref for Scale {

    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.scale
    }

}

impl <T> Add<T> for Scale where T: Into<f64> {

    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.scale + f)
    }

}

impl <T> Sub<T> for Scale where T: Into<f64> {

    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.scale - f)
    }

}

impl <T> Mul<T> for Scale where T: Into<f64> {

    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.scale * f)
    }

}

impl <T> Div<T> for Scale where T: Into<f64> {

    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.scale / f)
    }

}
