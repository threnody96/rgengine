use ::util::{ NoOption };

#[derive(Clone)]
pub struct RunActionOption {
    pub name: Option<String>
}

impl Default for RunActionOption {

    fn default() -> Self {
        RunActionOption {
            name: None
        }
    }

}

impl From<&RunActionOption> for RunActionOption {

    fn from(f: &RunActionOption) -> Self {
        f.clone()
    }

}

impl From<&str> for RunActionOption {

    fn from(f: &str) -> Self {
        RunActionOption {
            name: Some(f.to_owned())
        }
    }

}

impl From<String> for RunActionOption {

    fn from(f: String) -> Self {
        Self::from(f.as_str())
    }

}

impl From<&String> for RunActionOption {

    fn from(f: &String) -> Self {
        Self::from(f.to_string())
    }

}

impl From<NoOption> for RunActionOption {

    fn from(_: NoOption) -> Self {
        Self::default()
    }

}
