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

impl From<&Size> for Size {

    fn from(f: &Size) -> Size {
        f.clone()
    }

}

impl From<(u32, u32)> for Size {

    fn from(f: (u32, u32)) -> Size {
        Size::new(f.0, f.1)
    }

}
