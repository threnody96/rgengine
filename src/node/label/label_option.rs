use ::util::{ director };
use sdl2::pixels::{ Color };
use sdl2::ttf::{ FontStyle };

#[derive(Clone)]
pub struct LabelOption {
    pub path: String,
    pub point: u16,
    pub color: Color,
    pub style: FontStyle,
}

impl Default for LabelOption {

    fn default() -> Self {
        match director(|d| d.default_label_option()) {
            Some(o) => { o },
            None => {
                Self {
                    path: "default.ttf".to_owned(),
                    point: 30,
                    color: Color::RGBA(255, 255,255, 255),
                    style: FontStyle::NORMAL
                }
            }
        }
    }

}