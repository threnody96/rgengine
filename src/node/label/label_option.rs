#[derive(Clone)]
pub struct LabelOption {
    width: Option<f32>,
    height: Option<f32>
}

impl Default for LabelOption {

    fn default() -> Self {
        LabelOption {
            width: None,
            height: None
        }
    }

}