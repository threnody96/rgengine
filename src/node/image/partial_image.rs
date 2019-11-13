use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ NodeDelegate, Node };
use ::node::image::{ Image };
use ::resource::{ Texture };
use ::util::{ director };
use ::util::parameter::{ Size, Rect, AnchorPoint, Point };

pub struct PartialImage {
    part: RefCell<Rect>
}

impl PartialImage {

    pub fn create<A, B>(path: A, part: B) -> Rc<Node<PartialImage>>
    where A: Into<String>, B: Into<Rect>
    {
        let image = Image::create(path);
        let n = Node::create(PartialImage {
            part: RefCell::new(part.into())
        });
        n.add_child(image, "image");
        n.build();
        n
    }

    fn build(&self) {
        let part = self.part.borrow();
        let i: Rc<Node<Image>> = self.get_child("image").unwrap();
        i.set_anchor_point(AnchorPoint::new(0.0, 0.0));
        i.set_position(Point::new(part.x(), part.y()));
    }

    fn set_part<A>(&self, part: A) where A: Into<Rect> {
        self.part.replace(part.into());
        self.updated();
    }

    fn updated(&self) {
        self.build();
        self.clear_cache();
    }

}

impl NodeDelegate for PartialImage {

    fn get_size(&self) -> Size {
        let part = self.part.borrow();
        Size::new(part.width(), part.height())
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self) { }

    fn render(&self) { }

}

