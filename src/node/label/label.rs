use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ Node, NodeLike, NodeDelegate };
use ::util::{ director, render };
use ::resource::{ RFont };
pub use sdl2::pixels::{ Color };
use sdl2::ttf::{ FontStyle };

pub struct Label {
    text: RefCell<String>,
    font: RFont,
    color: Color
}

impl Label {

    pub fn create(text: &str, font_path: &str, color: &Color) -> Rc<Node<Self>> {
        let font = director(|d| d.load_font(font_path, 15, FontStyle::NORMAL));
        Node::create(|| {
            Self {
                text: RefCell::new(text.to_owned()),
                font: font.clone(),
                color: color.clone()
            }
        })
    }

    pub fn set_text(&self, text: &str) {
        self.text.replace(text.to_owned());
    }

}

impl NodeDelegate for Label {

    fn update(&self) { }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
        let text = self.text.borrow().clone();
        self.render_label(&parent, &text, &self.font, &self.color);
    }

}
