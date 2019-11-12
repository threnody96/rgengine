use ::util::{ NoOption };

#[derive(Clone)]
pub struct AddChildOption {
    pub z_index: i32,
    pub name: Option<String>
}

impl Default for AddChildOption {

    fn default() -> Self {
        Self {
            z_index: 0,
            name: None
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

impl From<&str> for AddChildOption {

    fn from(f: &str) -> AddChildOption {
        AddChildOption {
            name: Some(f.to_owned()),
            ..Default::default()
        }
    }

}

impl From<String> for AddChildOption {

    fn from(f: String) -> AddChildOption {
        Self::from(&f)
    }

}

impl From<&String> for AddChildOption {

    fn from(f: &String) -> AddChildOption {
        Self::from(f.to_string())
    }

}

impl From<NoOption> for AddChildOption {

    fn from(_: NoOption) -> AddChildOption {
        AddChildOption::default()
    }

}
