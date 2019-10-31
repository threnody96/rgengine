#[derive(Clone, Eq, PartialEq, Hash, Copy)]
pub enum ResolutionPolicy {
    ExactFit,
    NoBorder,
    FixedWidth,
    FixedHeight,
    ShowAll
}

