use ::util::{ NoOption };

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

impl From<&AddChildOption> for AddChildOption {

    fn from(f: &AddChildOption) -> AddChildOption {
        f.clone()
    }

}

impl From<i32> for AddChildOption {

    fn from(f: i32) -> AddChildOption {
        AddChildOption {
            z_index: f,
            ..Default::default()
        }
    }

}

impl From<(i32, &str)> for AddChildOption {

    fn from(f: (i32, &str)) -> AddChildOption {
        AddChildOption {
            z_index: f.0,
            tag: Some(f.1.to_string())
        }
    }

}

impl From<(i32, String)> for AddChildOption {

    fn from(f: (i32, String)) -> AddChildOption {
        AddChildOption {
            z_index: f.0,
            tag: Some(f.1.clone())
        }
    }

}

impl From<NoOption> for AddChildOption {

    fn from(_: NoOption) -> AddChildOption {
        AddChildOption::default()
    }

}
