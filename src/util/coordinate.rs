use ::util::{ Validation, FuzzyArg };
pub use sdl2::rect::{ Point, Rect };

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

#[derive(Clone)]
pub struct AnchorPoint {
    x: f32,
    y: f32,
}

impl AnchorPoint {

    pub fn new(x: f32, y: f32) -> Self {
        let s = Self { x: x, y: y};
        s.validate()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

}

impl Validation for AnchorPoint {

    type Output = AnchorPoint;

    fn validate(self) -> AnchorPoint {
        if self.x < 0.0 || self.x > 1.0 || self.y < 0.0 || self.y > 1.0 {
            panic!(format!("invalid anchor_point: {}, {}", self.x, self.y));
        }
        self
    }

}

impl Default for AnchorPoint {

    fn default() -> Self {
        AnchorPoint::new(0.5, 0.5)
    }

}
