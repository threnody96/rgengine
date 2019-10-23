use std::rc::Rc;
use ::node::{ Node, NodeDelegate };
use ::node::label::{PrettyLabel, LabelTextOption };
use ggez::{ Context };

pub struct LabelText {
    text: String,
    ruby: Option<PrettyLabel>,
    option: LabelTextOption
}

impl LabelText {

    pub fn create_by_text(text: String, option: LabelTextOption) -> Vec<Rc<Node<Self>>> {
        let mut output: Vec<Rc<Node<Self>>> = Vec::new();
        let normalized_text = Self::replace_escape_value(text);
        let texts: Vec<&str> = normalized_text.split("").collect();
        for t in texts {
            if t == "" { continue; }
            output.push(Node::create(|| {
                LabelText {
                    text: t.to_owned(),
                    ..Default::default()
                }
            }));
        }
        output
    }

    fn replace_escape_value(text: String) -> String {
        text.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
    }

}

impl NodeDelegate for LabelText {

    fn update(&self) {}

    fn render(&self) {
    }

}

impl Default for LabelText {

    fn default() -> Self {
        Self {
            text: "".to_owned(),
            ruby: None,
            option: LabelTextOption::default()
        }
    }

}
