use ggez::graphics::{ Font, Scale, Color };

pub enum LabelTag {
    Box { width: Option<f32>, height: Option<f32> },
    Option { font: Option<Font>, absolute_scale: Option<Scale>, scale: Option<Scale>, color: Option<Color>, ruby: Option<String> },
}

