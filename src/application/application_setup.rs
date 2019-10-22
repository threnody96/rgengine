use ::node::{ LabelTextOption };
use ::util::{ Size };
use ggez::conf::{ WindowSetup, WindowMode };

pub struct ApplicationSetup {
    pub width: f32,
    pub height: f32,
    pub title: String,
    pub icon: String,
    pub default_label_option: LabelTextOption
}

impl ApplicationSetup {

    pub fn generate_window_mode(&self) -> WindowMode {
        WindowMode {
            width: self.width,
            height: self.height,
            ..Default::default()
        }
    }

    pub fn generate_window_setup(&self) -> WindowSetup {
        WindowSetup {
            title: self.title.clone(),
            icon: self.icon.clone(),
            ..Default::default()
        }
    }

    pub fn generate_window_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height
        }
    }

}

impl Default for ApplicationSetup {

    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            title: "".to_owned(),
            icon: "".to_owned(),
            default_label_option: LabelTextOption::default()
        }
    }

}