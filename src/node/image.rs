use std::rc::Rc;
use ::node::{ NodeDelegate, Node, NodeLike };
use ::resource::{ RTexture };
use ::util::{ director, Size };

pub struct Image {
    image: Rc<RTexture>
}

impl Image {

    pub fn create(path: &str) -> Rc<Node<Image>> {
        Node::create(|| {
            Image { image: director(|d| d.load_texture(path)) }
        })
    }

}

impl NodeDelegate for Image {

    fn get_size(&self) -> Size {
        self.image.size()
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self, _parent: Rc<dyn NodeLike>) { }

    fn render(&self, _parent: Rc<dyn NodeLike>) {
        self.render_texture(self.image.clone());
    }

}

