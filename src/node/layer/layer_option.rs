use ::util::{ director, Size, FuzzyArg, NoOption };

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

impl FuzzyArg<LayerOption> for Size {

    fn take(&self) -> LayerOption {
        LayerOption {
            size: Some(self.clone()),
            ..Default::default()
        }
    }

}

impl FuzzyArg<LayerOption> for &Size {

    fn take(&self) -> LayerOption {
        LayerOption {
            size: Some((*self).clone()),
            ..Default::default()
        }
    }

}

impl FuzzyArg<LayerOption> for NoOption {

    fn take(&self) -> LayerOption {
        LayerOption::default()
    }

}
