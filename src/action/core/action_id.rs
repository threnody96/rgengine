#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ActionId {
    id: String
}

impl ActionId {

    pub fn new(id: String) -> Self {
        Self { id: id }
    }

}
