use std::ops::{ Add, Sub, Mul, Div, Deref };

#[derive(Clone, Copy)]
pub struct Rotation {
    rotation: f64
}

impl Rotation {

    pub fn new(rotation: f64) -> Self {
        let mut r = rotation % 360.0;
        if r < 0.0 { r += 360.0 }
        Self {
            rotation: r
        }
    }

}

impl Default for Rotation {

    fn default() -> Self {
        Self::new(0.0)
    }

}

impl From<Rotation> for f64 {

    fn from(f: Rotation) -> f64 {
        f.rotation
    }

}

impl From<f64> for Rotation {

    fn from(f: f64) -> Self {
        Self::new(f)
    }

}

impl Deref for Rotation {

    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.rotation
    }

}

impl <T> Add<T> for Rotation where T: Into<f64> {

    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.rotation + f)
    }

}

impl <T> Sub<T> for Rotation where T: Into<f64> {

    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.rotation - f)
    }

}

impl <T> Mul<T> for Rotation where T: Into<f64> {

    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.rotation * f)
    }

}

impl <T> Div<T> for Rotation where T: Into<f64> {

    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.rotation / f)
    }

}
