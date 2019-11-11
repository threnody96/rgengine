use std::ops::{ Add, Sub, Mul, Div, Deref };

#[derive(Clone, Copy)]
pub struct Opacity {
    opacity_rate: f64,
    opacity: u8
}

impl Opacity {

    pub fn new(opacity: f64) -> Self {
        let o = if opacity < 0.0 {
            0.0
        } else if opacity > 1.0 {
            1.0
        } else {
            opacity
        };
        Self {
            opacity_rate: o,
            opacity: (o * 255.0).round() as u8
        }
    }

    pub fn opacity_rate(&self) -> f64 {
        self.opacity_rate
    }

    pub fn is_translucence(&self) -> bool {
        self.opacity < 255
    }

    pub fn is_transparent(&self) -> bool {
        self.opacity == 0
    }

    pub fn is_untransparent(&self) -> bool {
        self.opacity == 255
    }

}

impl Default for Opacity {

    fn default() -> Self {
        Self::new(1.0)
    }

}

impl From<f64> for Opacity {

    fn from(f: f64) -> Self {
        Self::new(f)
    }

}

impl From<Opacity> for f64 {

    fn from(f: Opacity) -> Self {
        f.opacity_rate
    }

}

impl Deref for Opacity {

    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.opacity
    }

}

impl <T> Add<T> for Opacity where T: Into<f64> {

    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.opacity_rate + f)
    }

}

impl <T> Sub<T> for Opacity where T: Into<f64> {

    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.opacity_rate - f)
    }

}

impl <T> Mul<T> for Opacity where T: Into<f64> {

    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.opacity_rate * f)
    }

}

impl <T> Div<T> for Opacity where T: Into<f64> {

    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let f = other.into();
        Self::from(self.opacity_rate / f)
    }

}
