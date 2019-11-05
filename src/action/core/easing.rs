pub fn linear() -> Box<Fn(f32) -> f32> {
    Box::new(|f| f)
}
