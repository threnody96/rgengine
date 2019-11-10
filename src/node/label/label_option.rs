use std::collections::HashMap;
use ::util::{ director, NoOption };
use ::util::parameter::{ FontStyle, Color };

#[derive(Clone)]
pub struct LabelOption {
    pub path: String,
    pub point: u16,
    pub color: Color,
    pub style: FontStyle,
    pub border: Option<Color>
}

impl LabelOption {

    pub fn parse(parent_option: &LabelOption, name: String, attrs: HashMap<String, String>) -> Self {
        let mut option = parent_option.clone();
        if &name != "style" { return option; }
        option = Self::attach_name(&option, attrs.get("name").cloned());
        Self::attach_scale(&mut option, attrs.get("scale").cloned());
        Self::attach_absolute_scale(&mut option, attrs.get("absolute-scale").cloned());
        Self::attach_font(&mut option, attrs.get("font").cloned());
        Self::attach_color(&mut option, attrs.get("color").cloned());
        Self::attach_border(&mut option, attrs.get("border").cloned());
        option
    }

    fn attach_name(option: &LabelOption, name: Option<String>) -> LabelOption {
        if let Some(n) = name {
            if let Some(l) = director::get_label_option(n) {
                return l;
            }
        }
        option.clone()
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

    fn attach_color(option: &mut LabelOption, color: Option<String>) {
        if let Ok(c) = Self::parse_color(color) {
            option.color = c;
        }
    }

    fn attach_border(option: &mut LabelOption, color: Option<String>) {
        if let Some(c) = color.clone() {
            if &c == "" {
                option.border = None;
                return;
            }
        }
        if let Ok(c) = Self::parse_color(color) {
            option.border = Some(c);
        }
    }

    fn parse_color(color: Option<String>) -> Result<Color, ()> {
        if color.is_none() { return Err(()); }
        let c = color.clone().unwrap();
        let color_codes: Vec<&str> = c.split(",").collect();
        if color_codes.len() != 3 && color_codes.len() != 4 { return Err(()); }
        let codes: (&str, &str, &str, &str) = (
            color_codes.get(0).cloned().ok_or(())?,
            color_codes.get(1).cloned().ok_or(())?,
            color_codes.get(2).cloned().ok_or(())?,
            color_codes.get(3).cloned().unwrap_or("255")
        );
        Ok(Color::RGBA(
            codes.0.parse::<u8>().map_err(|_| ())?,
            codes.1.parse::<u8>().map_err(|_| ())?,
            codes.2.parse::<u8>().map_err(|_| ())?,
            codes.3.parse::<u8>().map_err(|_| ())?,
        ))
    }

}

impl Default for LabelOption {

    fn default() -> Self {
        match director::default_label_option() {
            Some(o) => { o },
            None => {
                Self {
                    path: "default.ttf".to_owned(),
                    point: 30,
                    color: Color::RGBA(255, 255,255, 255),
                    style: FontStyle::normal(),
                    border: None
                }
            }
        }
    }

}

impl From<String> for LabelOption {

    fn from(f: String) -> LabelOption {
        match director::get_label_option(f.clone()) {
            None => {
                LabelOption {
                    path: f,
                    ..Default::default()
                }
            },
            Some(l) => {
                l
            }
        }
    }

}

impl From<&str> for LabelOption {

    fn from(f: &str) -> LabelOption {
        LabelOption::from(f.to_owned())
    }

}

impl From<&String> for LabelOption {

    fn from(f: &String) -> LabelOption {
        LabelOption::from(f.to_string())
    }

}

impl From<&LabelOption> for LabelOption {

    fn from(f: &LabelOption) -> LabelOption {
        f.clone()
    }

}

impl From<u16> for LabelOption {

    fn from(f: u16) -> LabelOption {
        LabelOption {
            point: f,
            ..Default::default()
        }
    }

}

impl From<Color> for LabelOption {

    fn from(f: Color) -> LabelOption {
        LabelOption {
            color: f,
            ..Default::default()
        }
    }

}

impl From<&Color> for LabelOption {

    fn from(f: &Color) -> LabelOption {
        Self::from(f.clone())
    }

}

impl From<FontStyle> for LabelOption {

    fn from(f: FontStyle) -> LabelOption {
        LabelOption {
            style: f,
            ..Default::default()
        }
    }

}

impl From<&FontStyle> for LabelOption {

    fn from(f: &FontStyle) -> LabelOption {
        Self::from(f.clone())
    }

}

impl From<NoOption> for LabelOption {

    fn from(_: NoOption) -> LabelOption {
        LabelOption::default()
    }

}

