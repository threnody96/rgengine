use std::rc::Rc;
use ::node::{ NodeDelegate, Node, NodeLike };
use ::resource::{ Texture };
use ::util::{ director };
use ::util::parameter::{ Size };

pub struct Image {
    image: Rc<Texture>
}

impl Image {

    pub fn create<A>(path: A) -> Rc<Node<Image>>
    where A: Into<String>
    {
        let p = path.into();
        Node::create(|| {
            Image { image: director::load_texture(&p) }
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

    fn update(&self) { }

    fn render(&self) {
        self.render_texture(self.image.clone());
    }

}

