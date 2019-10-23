use std::collections::HashMap;
use ::util::{ must, director };
use ggez::{ Context };
use ggez::graphics::{ Scale, Color, Font };

#[derive(Clone)]
pub struct LabelTextOption {
    pub size: f32,
    pub size_name: String,
    pub size_magnification: f32,
    pub color: Color,
    pub color_name: String,
    pub font: String,
}

impl LabelTextOption {

    // pub fn parse(parent_option: &Self, tag: String, attrs: HashMap<String, String>) -> Self {
    //     let mut option = parent_option.clone();
    //     if &tag != "meta" { return option; }
    //     Self::attach_scale(&mut option, attrs.get("scale").map(|t| t.as_ref()));
    //     Self::attach_absolute_scale(&mut option, attrs.get("absolute-scale").map(|t| t.as_ref()));
    //     Self::attach_font(&mut option, attrs.get("font").map(|t| t.as_ref()));
    //     Self::attach_color(&mut option, attrs.get("color").map(|t| t.as_ref()));
    //     option
    // }

    pub fn size(&self) -> Scale {
        let mut scale = if &self.size_name == "" {
            Scale::uniform(self.size)
        } else {
            must(director(|d| d.get_font_size(self.size_name.clone())).ok_or(format!("unknown scale: {}", &self.size_name)))
        };
        scale.x *= self.size_magnification;
        scale.y *= self.size_magnification;
        scale
    }

    pub fn color(&self) -> Color {
        let err = format!("unknown color: {}", &self.color_name);
        if &self.color_name == "" {
            self.color.clone()
        } else {
            match director(|d| d.get_color(self.color_name.clone())) {
                Some(c) => { c },
                None => {
                    let color_codes: Vec<&str> = self.color_name.split(",").collect();
                    if color_codes.len() != 3 && color_codes.len() != 4 {
                        must::<String, String>(Err(err.clone()));
                    }
                    let codes: (&str, &str, &str, &str) = (
                        must(color_codes.get(0).cloned().ok_or(err.clone())),
                        must(color_codes.get(1).cloned().ok_or(err.clone())),
                        must(color_codes.get(2).cloned().ok_or(err.clone())),
                        color_codes.get(3).cloned().unwrap_or("255")
                    );
                    Color::from_rgba(
                        must(codes.0.parse::<u8>()),
                        must(codes.1.parse::<u8>()),
                        must(codes.2.parse::<u8>()),
                        must(codes.3.parse::<u8>()),
                    )
                }
            }
        }
    }

    pub fn font(&self) -> String {
        if &self.font == "" {
            "".to_owned()
        } else {
            director(|d| d.get_font(self.font.clone())).unwrap_or(self.font.clone())
        }
    }

    // fn attach_scale(option: &mut Self, value: Option<&str>) {
    //     if value.is_none() { return; }
    //     let v = must(value.unwrap().parse::<f32>());
    //     option.scale.x *= v;
    //     option.scale.y *= v;
    // }

    // fn attach_absolute_scale(option: &mut Self, value: Option<&str>) {
    //     if value.is_none() { return; }
    //     let val = value.unwrap();
    //     match val.parse::<f32>() {
    //         Ok(v) => {
    //             option.scale = Scale::uniform(v);
    //         },
    //         Err(_) => {
    //             let scale = director(|d| d.get_font_size(val.to_owned()));
    //             option.scale = must(scale.ok_or(format!("unknown scale: {}", val)));
    //         }
    //     }
    // }

    // fn attach_font(option: &mut Self, value: Option<&str>) {
    //     if value.is_none() { return; }
    //     let val = value.unwrap();
    //     if val == "default" {
    //         option.font = None;
    //     } else {
    //         match director(|d| d.get_font(val.to_owned())) {
    //             Some(p) => { option.font = Some(p); },
    //             None => { option.font = Some(val.to_owned()); }
    //         }
    //     }
    // }

    // fn attach_color(option: &mut Self, value: Option<&str>) {
    //     if value.is_none() { return; }
    //     let val = value.unwrap();
    //     let err = format!("unknown color: {}", val);
    //     match director(|d| d.get_color(val.to_owned())) {
    //         Some(c) => { option.color = c; },
    //         None => {
    //             let color_codes: Vec<&str> = val.split(",").collect();
    //             if color_codes.len() != 3 && color_codes.len() != 4 {
    //                 must::<String, String>(Err(err.clone()));
    //             }
    //             let codes: (&str, &str, &str, &str) = (
    //                 must(color_codes.get(0).cloned().ok_or(err.clone())),
    //                 must(color_codes.get(1).cloned().ok_or(err.clone())),
    //                 must(color_codes.get(2).cloned().ok_or(err.clone())),
    //                 color_codes.get(3).cloned().unwrap_or("255")
    //             );
    //             option.color = Color::from_rgba(
    //                 must(codes.0.parse::<u8>()),
    //                 must(codes.1.parse::<u8>()),
    //                 must(codes.2.parse::<u8>()),
    //                 must(codes.3.parse::<u8>()),
    //             );
    //         }
    //     }
    // }

    fn application_default() -> Self {
        director(|d| d.get_default_label_option())
    }

}

impl Default for LabelTextOption {

    fn default() -> Self {
        Self {
            size: 10.0,
            size_name: "".to_owned(),
            size_magnification: 1.0,
            color: Color::from_rgba(255, 255,255, 255),
            color_name: "".to_owned(),
            font: "".to_owned(),
        }
    }

}
