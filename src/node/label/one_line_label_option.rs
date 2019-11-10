use ::node::label::{ LabelOption };
use ::util::{ NoOption };
use ::util::parameter::{ FontStyle, Color };

#[derive(Clone)]
pub struct OneLineLabelOption {
    pub path: String,
    pub point: u16,
    pub color: Color,
    pub style: FontStyle,
}

impl Default for OneLineLabelOption {

    fn default() -> Self {
        OneLineLabelOption::from(LabelOption::default())
    }

}

impl From<&OneLineLabelOption> for OneLineLabelOption {

    fn from(f: &OneLineLabelOption) -> OneLineLabelOption {
        f.clone()
    }

}

impl From<LabelOption> for OneLineLabelOption {

    fn from(f: LabelOption) -> OneLineLabelOption {
        Self {
            path: f.path.clone(),
            point: f.point.clone(),
            color: f.color.clone(),
            style: f.style.clone()
        }
    }

}

impl From<&LabelOption> for OneLineLabelOption {

    fn from(f: &LabelOption) -> OneLineLabelOption {
        Self::from(f.clone())
    }

}

impl From<u16> for OneLineLabelOption {

    fn from(f: u16) -> OneLineLabelOption {
        OneLineLabelOption::from(LabelOption::from(f))
    }

}

impl From<Color> for OneLineLabelOption {

    fn from(f: Color) -> OneLineLabelOption {
        OneLineLabelOption::from(LabelOption::from(f))
    }

}

impl From<&Color> for OneLineLabelOption {

    fn from(f: &Color) -> OneLineLabelOption {
        Self::from(f.clone())
    }

}

impl From<FontStyle> for OneLineLabelOption {

    fn from(f: FontStyle) -> OneLineLabelOption {
        OneLineLabelOption::from(LabelOption::from(f))
    }

}

impl From<&FontStyle> for OneLineLabelOption {

    fn from(f: &FontStyle) -> OneLineLabelOption {
        Self::from(f.clone())
    }

}

impl From<NoOption> for OneLineLabelOption {

    fn from(_: NoOption) -> OneLineLabelOption {
        OneLineLabelOption::default()
    }

}

