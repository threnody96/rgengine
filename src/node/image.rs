use std::rc::Rc;
use ::node::{ NodeDelegate, Node };
use ::resource::{ RTexture };
use ::util::{ director, render };

pub struct Image {
    image: RTexture
}

impl Image {

    pub fn create(path: &str) -> Rc<Node<Image>> {
        Node::create(|| {
            Image { image: director(|d| d.load_texture(path)) }
        })
    }

}

impl NodeDelegate for Image {

    fn update(&self) { }

    fn render(&self) {
        render(|r| r.render_texture(&self.image));
    }

}

