use ::scene::{ Node };

pub struct Scene {
    children: Vec<Box<dyn Node>>
}

impl Scene {

    pub fn create() -> Self {
        Self {
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, node: Box<dyn Node>) {
        self.children.push(node);
    }

}