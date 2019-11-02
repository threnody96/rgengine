use std::collections::HashMap;
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

impl LabelOption {

    pub fn parse(parent_option: &LabelOption, name: String, attrs: HashMap<String, String>) -> Self {
        let mut option = parent_option.clone();
        if &name != "style" { return option; }
        Self::attach_scale(&mut option, attrs.get("scale").cloned());
        Self::attach_absolute_scale(&mut option, attrs.get("absolute-scale").cloned());
        Self::attach_font(&mut option, attrs.get("font").cloned());
        Self::attach_color(&mut option, attrs.get("color").cloned());
        option
    }

    fn attach_scale(option: &mut LabelOption, scale: Option<String>) {
        if scale.is_none() { return; }
        if let Ok(v) = scale.unwrap().parse::<f32>() {
            option.point = (option.point as f32 * v) as u16;
        }
    }

    fn attach_absolute_scale(option: &mut LabelOption, scale: Option<String>) {
        if scale.is_none() { return; }
        if let Ok(v) = scale.unwrap().parse::<u16>() {
            option.point = v;
        }
    }

    fn attach_font(option: &mut LabelOption, font: Option<String>) {
        if font.is_none() { return; }
        option.path = font.unwrap();
    }

    fn attach_color(option: &mut LabelOption, color: Option<String>) -> Result<(), ()> {
        if color.is_none() { return Ok(()); }
        let c = color.clone().unwrap();
        let color_codes: Vec<&str> = c.split(",").collect();
        if color_codes.len() != 3 && color_codes.len() != 4 { return Err(()); }
        let codes: (&str, &str, &str, &str) = (
            color_codes.get(0).cloned().ok_or(())?,
            color_codes.get(1).cloned().ok_or(())?,
            color_codes.get(2).cloned().ok_or(())?,
            color_codes.get(3).cloned().unwrap_or("255")
        );
        option.color = Color::RGBA(
            codes.0.parse::<u8>().map_err(|_| ())?,
            codes.1.parse::<u8>().map_err(|_| ())?,
            codes.2.parse::<u8>().map_err(|_| ())?,
            codes.3.parse::<u8>().map_err(|_| ())?,
        );
        Ok(())
    }

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