use ::util::{ NoOption };
use ::util::parameter::{ Color };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SquareOption {
    pub color: Color
}

impl Default for SquareOption {

    fn default() -> Self {
        Self {
            color: Color::RGBA(255, 255, 255,0)
        }
    }

}

impl From<NoOption> for SquareOption {

    fn from(_: NoOption) -> Self {
        Self::default()
    }

}
