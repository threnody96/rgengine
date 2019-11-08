#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TransitionStatus {
    Wait,
    Processing,
    Finished
}