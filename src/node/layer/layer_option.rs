use ::util::{ NoOption };
use ::util::parameter::{ Size };

#[derive(Clone)]
pub struct LayerOption {
    pub size: Option<Size>
}

impl Default for LayerOption {

    fn default() -> Self {
        Self {
            size: None
        }
    }

}

impl From<Size> for LayerOption {

    fn from(f: Size) -> LayerOption {
        LayerOption {
            size: Some(f),
            ..Default::default()
        }
    }

}

impl From<&Size> for LayerOption {

    fn from(f: &Size) -> LayerOption {
        Self::from(f.clone())
    }

}

impl From<NoOption> for LayerOption {

    fn from(_: NoOption) -> LayerOption {
        LayerOption::default()
    }

}
