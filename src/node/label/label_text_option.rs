use std::collections::HashMap;
use ::util::{ must, director };
use ggez::{ Context };
use ggez::graphics::{ Scale, Color, Font };

#[derive(Clone)]
pub struct LabelTextOption {
    scale: Scale,
    color: Color,
    font: Option<String>
}

impl LabelTextOption {

    pub fn parse(parent_option: &Self, tag: String, attrs: HashMap<String, String>) -> Self {
        let mut option = parent_option.clone();
        if &tag != "meta" { return option; }
        Self::attach_scale(&mut option, attrs.get("scale").map(|t| t.as_ref()));
        Self::attach_absolute_scale(&mut option, attrs.get("absolute-scale").map(|t| t.as_ref()));
        if attrs.get("font").is_some() {  }
        if attrs.get("color").is_some() { }
        if attrs.get("ruby").is_some() { }
        option
    }

    fn attach_scale(option: &mut Self, value: Option<&str>) {
        if value.is_none() { return; }
        let v = must(value.unwrap().parse::<f32>());
        option.scale.x *= v;
        option.scale.y *= v;
    }

    fn attach_absolute_scale(option: &mut Self, value: Option<&str>) {
        if value.is_none() { return; }
        let val = value.unwrap();
        match val.parse::<f32>() {
            Ok(v) => {
                option.scale = Scale::uniform(v);
            },
            Err(_) => {
                let scale = director(|d| d.get_font_size(val.to_owned()));
                option.scale = must(scale.ok_or(format!("unknown scale: {}", val)));
            }
        }
    }

}

impl Default for LabelTextOption {

    fn default() -> Self {
        Self {
            scale: Scale::uniform(10.0),
            color: Color::from_rgba(255, 255,255, 255),
            font: None
        }
    }

}
