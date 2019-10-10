use ::scene::node::{ Layer };

pub struct Scene {
    children: Vec<Layer>
}

impl Scene {

    pub fn create() -> Self {
        Self {
            children: Vec::new()
        }
    }

}