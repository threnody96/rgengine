use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ Node, NodeDelegate };
use ::node::label::{ OneLineLabelOption };
use ::util::{ director };
use ::util::parameter::{ Size };
use ::resource::{ Font };
pub use sdl2::pixels::{ Color };

pub struct OneLineLabel {
    size: RefCell<Option<Size>>,
    text: RefCell<String>,
    font: RefCell<Rc<Font>>,
    option: RefCell<OneLineLabelOption>
}

impl OneLineLabel {

    pub fn create<A, B>(text: A, option: B) -> Rc<Node<Self>>
    where
        A: Into<String>,
        B: Into<OneLineLabelOption>
    {
        let t = text.into();
        let o = option.into();
        let font = director::load_font(&o);
        let n = Node::create(|| {
            Self {
                size: RefCell::new(None),
                text: RefCell::new(Self::normalize_text(&t)),
                font: RefCell::new(font.clone()),
                option: RefCell::new(o.clone())
            }
        });
        n.updated();
        n
    }

    pub fn set_text<A>(&self, text: A)
    where A: Into<String>
    {
        let t = text.into();
        self.text.replace(Self::normalize_text(&t));
        self.updated();
    }

    pub fn set_point<A>(&self, point: A)
    where A: Into<u16>
    {
        {
            let mut option = self.option.borrow_mut();
            option.point = point.into();
            self.font.replace(director::load_font(&option.clone()));
        }
        self.updated();
    }

    fn updated(&self) {
        let text = self.text.borrow().clone();
        let font = self.font.borrow().clone();
        let size = director::measure_label_size(&text, font.clone());
        self.size.replace(Some(size));
        self.clear_cache();
    }

    fn normalize_text(text: &str) -> String {
        (if text == "" { " " } else { text }).to_owned()
    }

}

impl NodeDelegate for OneLineLabel {

    fn get_size(&self) -> Size {
        self.size.borrow().clone().unwrap()
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self) { }

    fn render(&self) {
        let text = self.text.borrow().clone();
        let font = self.font.borrow().clone();
        let option = self.option.borrow().clone();
        self.render_label(&text, font, &option.color);
    }

}
