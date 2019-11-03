use ::util::{ NoOption, FuzzyArg };

#[derive(Clone)]
pub struct AddChildOption {
    pub z_index: i32,
    pub tag: Option<String>
}

impl Default for AddChildOption {

    fn default() -> Self {
        Self {
            z_index: 0,
            tag: None
        }
    }

}

impl FuzzyArg<AddChildOption> for AddChildOption {

    fn take(&self) -> AddChildOption {
        self.clone()
    }

}

impl FuzzyArg<AddChildOption> for &AddChildOption {

    fn take(&self) -> AddChildOption {
        (*self).clone()
    }

}

impl FuzzyArg<AddChildOption> for i32 {

    fn take(&self) -> AddChildOption {
        AddChildOption {
            z_index: *self,
            ..Default::default()
        }
    }

}

impl FuzzyArg<AddChildOption> for (i32, &str) {

    fn take(&self) -> AddChildOption {
        AddChildOption {
            z_index: self.0,
            tag: Some(self.1.to_string())
        }
    }

}

impl FuzzyArg<AddChildOption> for (i32, String) {

    fn take(&self) -> AddChildOption {
        AddChildOption {
            z_index: self.0,
            tag: Some(self.1.clone())
        }
    }

}

impl FuzzyArg<AddChildOption> for NoOption {

    fn take(&self) -> AddChildOption {
        AddChildOption::default()
    }

}
