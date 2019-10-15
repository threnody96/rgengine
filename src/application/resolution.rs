use ::util::{ Size };

#[derive(Clone)]
pub enum ResolutionPolicy {
    ExactFit,
    NoBorder,
    ShowAll
}

#[derive(Clone)]
pub struct ResolutionSize {
    pub size: Size,
    pub policy: ResolutionPolicy
}