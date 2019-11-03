use ::util::{ Validation, FuzzyArg };

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

impl FuzzyArg<AnchorPoint> for AnchorPoint {

    fn take(&self) -> AnchorPoint {
        self.clone()
    }

}

impl FuzzyArg<AnchorPoint> for &AnchorPoint {

    fn take(&self) -> AnchorPoint {
        (*self).clone()
    }

}

impl FuzzyArg<AnchorPoint> for (f32, f32) {

    fn take(&self) -> AnchorPoint {
        AnchorPoint::new(self.0, self.1)
    }

}