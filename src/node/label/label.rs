use std::rc::Rc;
use ::node::{ Node, NodeDelegate, LabelTextOption };
use ::util::{ director, Point };
use ggez::{ Context };
use ggez::graphics::{ draw, Text, TextFragment };

pub struct Label {
    text: String,
    option: LabelTextOption
}

impl Label {

    fn create(text: String, option: LabelTextOption) -> Rc<Node<Self>> {
        director(|d| d.preload_font(option.font()));
        Node::create(|| {
            Self {
                text: text.clone(),
                option: option.clone()
            }
        })
    }

    fn text(&self) -> Text {
        Text::new(TextFragment {
            text: self.text.clone(),
            color: Some(self.option.color()),
            font: Some((*director(|d| d.load_font(self.option.font()))).clone()),
            scale: Some(self.option.size()),
            ..Default::default()
        })
    }

}

impl NodeDelegate for Label {

    fn update(&self) { }

    fn render(&self, ctx: &mut Context) {
        let text = self.text();
        // draw(ctx, &text, (Point { x: 0.0, y: 0.0 }));
    }

}
