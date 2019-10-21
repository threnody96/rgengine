#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: String
}

impl NodeId {

    pub fn new(id: String) -> Self {
        Self { id: id }
    }

}