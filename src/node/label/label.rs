use std::rc::Rc;
use std::collections::HashMap;
use ::node::{ Node, NodeDelegate, AddChildOption };
use ::node::label::{ LabelText, LabelTextOption };
use ::util::{ director };
use ggez::{ Context };
use ggez::graphics::{ Scale, Color };
use html5ever::{ parse_document, QualName };
use html5ever::driver::{ Parser, ParseOpts };
use html5ever::rcdom::{ Handle, RcDom, NodeData };
use html5ever::tendril::TendrilSink;

pub struct Label { }

impl Label {

    pub fn create(text: String) -> Rc<Node<Self>> {
        let mut parser = parse_document(RcDom::default(), ParseOpts::default());
        let label = Node::create(|| Label { });
        let dom = parser.one(text.as_ref());
        let option = director(|d| d.get_default_label_option() );
        for t in Self::parse(&dom.document, &option) {
            label.add_child(t, AddChildOption::default());
        }
        label
    }

    fn parse(node: &Handle, parent_option: &LabelTextOption) -> Vec<Rc<Node<LabelText>>> {
        match node.data {
            NodeData::Text { ref contents } => {
                LabelText::create_by_text(contents.borrow().to_string(), parent_option.clone())
            },
            NodeData::Element { ref name, ref attrs, .. } => {
                let mut attr_map: HashMap<String, String> = HashMap::new();
                for attr in attrs.borrow().iter() {
                    attr_map.insert(attr.name.local.to_string().to_lowercase(), attr.value.to_string());
                }
                let next_option = LabelTextOption::parse(parent_option, name.local.to_string().to_lowercase(), attr_map);
                Self::parse_children(node, &next_option)
            },
            _ => { Self::parse_children(node, parent_option) }
        }
    }

    fn parse_children(node: &Handle, parent_option: &LabelTextOption) -> Vec<Rc<Node<LabelText>>> {
        let mut output: Vec<Rc<Node<LabelText>>> = Vec::new();
        for child in node.children.borrow().iter() {
            let mut texts = Self::parse(node, parent_option);
            output.append(&mut texts);
        }
        output
    }

}

impl NodeDelegate for Label {

    fn update(&self) { }

    fn render(&self, ctx: &mut Context) { }

}
