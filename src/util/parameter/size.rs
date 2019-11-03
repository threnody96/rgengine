use ::util::{ FuzzyArg };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Size {
    width: u32,
    height: u32
}

impl Size {

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

}

impl FuzzyArg<Size> for Size {

    fn take(&self) -> Size {
        self.clone()
    }

}

impl FuzzyArg<Size> for &Size {

    fn take(&self) -> Size {
        (*self).clone()
    }

}

impl FuzzyArg<Size> for (u32, u32) {

    fn take(&self) -> Size {
        Size::new(self.0, self.1)
    }

}
