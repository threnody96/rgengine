use ::util::{ Size };

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ResolutionPolicy {
    ExactFit,
    NoBorder,
    FixedWidth,
    FixedHeight,
    ShowAll
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ResolutionSize {
    pub size: Size,
    pub policy: ResolutionPolicy
}

impl Default for ResolutionSize {

    fn default() -> Self {
        ResolutionSize {
            size: Size { width: 800, height: 600 },
            policy: ResolutionPolicy::ExactFit
        }
    }

}
