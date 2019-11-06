use ::node::{ NodeId };

#[derive(Clone)]
pub struct NodeChild {
    pub id: NodeId,
    pub z_index: i32,
    pub inner_z_index: u32,
}
