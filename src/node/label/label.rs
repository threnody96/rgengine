use std::rc::Rc;
use ::node::{ Node, NodeDelegate, LabelTextOption };
use ::util::{ director, draw, Point };
use ggez::graphics::{ Text, TextFragment, DrawParam };

pub struct Label {
    text: Text
}

impl Label {

    fn create(text: String, option: LabelTextOption) -> Rc<Node<Self>> {
        Node::create(|| {
            Self {
                text: Text::new(TextFragment {
                    text: text.clone(),
                    color: Some(option.color()),
                    font: Some((*director(|d| d.load_font(option.font()))).clone()),
                    scale: Some(option.size()),
                    ..Default::default()
                })
            }
        })
    }

}

impl NodeDelegate for Label {

    fn update(&self) { }

    fn render(&self) {
        draw(&self.text, DrawParam {
            dest: Point { x: 0.0, y: 0.0 },
            ..Default::default()
        });
    }

}
