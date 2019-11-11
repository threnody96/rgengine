use ::util::{ NoOption };
use ::util::parameter::{ Color };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RoundOption {
    pub color: Color
}

impl Default for RoundOption {

    fn default() -> Self {
        Self {
            color: Color::RGBA(255, 255, 255,0)
        }
    }

}

impl From<NoOption> for RoundOption {

    fn from(_: NoOption) -> Self {
        Self::default()
    }

}