use ::util::{ NoOption };

#[derive(Clone)]
pub struct AddChildOption {
    pub z_index: i32,
}

impl Default for AddChildOption {

    fn default() -> Self {
        Self {
            z_index: 0,
        }
    }

}

impl From<&AddChildOption> for AddChildOption {

    fn from(f: &AddChildOption) -> AddChildOption {
        f.clone()
    }

}

impl From<i32> for AddChildOption {

    fn from(f: i32) -> AddChildOption {
        AddChildOption {
            z_index: f,
        }
    }

}

impl From<NoOption> for AddChildOption {

    fn from(_: NoOption) -> AddChildOption {
        AddChildOption::default()
    }

}
