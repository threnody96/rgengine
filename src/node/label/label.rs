use std::rc::Rc;
use std::cell::RefCell;
use ::node::{ Node, NodeLike, NodeDelegate };
use ::util::{ director, render, Size };
use ::resource::{ RFont };
pub use sdl2::pixels::{ Color };
use sdl2::ttf::{ FontStyle };

pub struct Label {
    size: RefCell<Option<Size>>,
    text: RefCell<String>,
    font: RefCell<Rc<RFont>>,
    color: Color
}

impl Label {

    pub fn create(text: &str, font_path: &str, color: &Color) -> Rc<Node<Self>> {
        let font = director(|d| d.load_font(font_path, 15, FontStyle::NORMAL));
        let n = Node::create(|| {
            Self {
                size: RefCell::new(None),
                text: RefCell::new(text.to_owned()),
                font: RefCell::new(font.clone()),
                color: color.clone()
            }
        });
        n.update_size();
        n
    }

    pub fn set_text(&self, text: &str) {
        self.text.replace(text.to_owned());
        self.update_size();
    }

    fn update_size(&self) {
        let text = self.text.borrow().clone();
        match text.as_str() {
            "" => { self.size.replace(Some(Size { width: 0, height: 0 })); },
            _ => {
                let font = self.font.borrow().clone();
                let size = render(|r| r.measure_label_size(&text, font.clone()));
                self.size.replace(Some(size));
            }
        }
    }

}

impl NodeDelegate for Label {

    fn get_size(&self) -> Size {
        self.size.borrow().clone().unwrap()
    }

    fn update(&self) { }

    fn render(&self, parent: Option<Rc<dyn NodeLike>>) {
        let text = self.text.borrow().clone();
        let font = self.font.borrow().clone();
        self.render_label(&parent, &text, font, &self.color);
    }

}
