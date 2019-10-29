use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ Node, NodeLike, NodeDelegate, LabelOption };
use ::util::{ director, render, Size };
use ::resource::{ RFont };
pub use sdl2::pixels::{ Color };
use sdl2::ttf::{ FontStyle };

pub struct OneLineLabel {
    size: RefCell<Option<Size>>,
    text: RefCell<String>,
    font: RefCell<Rc<RFont>>,
    option: RefCell<LabelOption>
}

impl OneLineLabel {

    pub fn create(text: &str, option: &LabelOption) -> Rc<Node<Self>> {
        let font = director(|d| d.load_font(option));
        let n = Node::create(|| {
            Self {
                size: RefCell::new(None),
                text: RefCell::new(Self::normalize_text(text)),
                font: RefCell::new(font.clone()),
                option: RefCell::new(option.clone())
            }
        });
        n.update_size();
        n
    }

    pub fn set_text(&self, text: &str) {
        self.text.replace(Self::normalize_text(text));
        self.update_size();
    }

    pub fn set_point(&self, point: u16) {
        {
            let mut option = self.option.borrow_mut();
            option.point = point;
            self.font.replace(director(|d| d.load_font(&option.clone())));
        }
        self.update_size();
    }

    fn update_size(&self) {
        let text = self.text.borrow().clone();
        let font = self.font.borrow().clone();
        let size = render(|r| r.measure_label_size(&text, font.clone()));
        self.size.replace(Some(size));
    }

    fn normalize_text(text: &str) -> String {
        (if text == "" { " " } else { text }).to_owned()
    }

}

impl NodeDelegate for OneLineLabel {

    fn get_size(&self) -> Size {
        self.size.borrow().clone().unwrap()
    }

    fn update(&self) { }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
        let text = self.text.borrow().clone();
        let font = self.font.borrow().clone();
        let option = self.option.borrow().clone();
        self.render_label(&parent, &text, font, &option.color);
    }

}
