use ::util::{ Validation, NoOption };

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
            panic!(format!("anchor_point が不正な値です: {}, {}", self.x, self.y));
        }
        self
    }

}

impl Default for AnchorPoint {

    fn default() -> Self {
        AnchorPoint::new(0.5, 0.5)
    }

}

impl From<&AnchorPoint> for AnchorPoint {

    fn from(f: &AnchorPoint) -> Self {
        f.clone()
    }

}

impl <A, B> From<(A, B)> for AnchorPoint
where
    A: Into<f32>,
    B: Into<f32>
{

    fn from(f: (A, B)) -> AnchorPoint {
        AnchorPoint::new(f.0.into(), f.1.into())
    }

}

impl From<NoOption> for AnchorPoint {

    fn from(_: NoOption) -> AnchorPoint {
        AnchorPoint::default()
    }

}
