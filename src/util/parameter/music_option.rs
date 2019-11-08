use ::util::NoOption;

pub struct MusicOption {
    pub loops: i32,
    pub fade_in: Option<i32>,
    pub position: Option<f64>
}

impl Default for MusicOption {

    fn default() -> Self {
        Self {
            loops: 0,
            fade_in: None,
            position: None
        }
    }

}

impl From<NoOption> for MusicOption {

    fn from(_: NoOption) -> Self {
        MusicOption::default()
    }

}

