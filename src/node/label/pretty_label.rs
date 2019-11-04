use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::max;
use ::node::{ Node, NodeDelegate, NodeLike, AddChildOption };
use ::node::label::{ OneLineLabel, LabelOption };
use ::util::parameter::{ Size, Point, AnchorPoint };
use html5ever::{ parse_document };
use html5ever::driver::{ ParseOpts };
use html5ever::rcdom::{ Handle, RcDom, NodeData };
use html5ever::tendril::TendrilSink;
use html5ever::interface::{ Attribute };

pub struct PrettyLabel {
    text: RefCell<String>,
    size: RefCell<Option<Size>>,
    labels: RefCell<Vec<Rc<Node<OneLineLabel>>>>
}

impl PrettyLabel {

    pub fn create<A>(text: A) -> Rc<Node<Self>>
    where A: Into<String>
    {
        let t = text.into();
        let n= Node::create(|| PrettyLabel {
            text: RefCell::new(t.clone()),
            size: RefCell::new(None),
            labels: RefCell::new(Vec::new())
        });
        n.build();
        n
    }

    pub fn set_text<A>(&self, text: A)
    where A: Into<String>
    {
        self.text.replace(text.into());
        self.updated();
    }

    fn build(&self) {
        let text = self.text.borrow().clone();
        let parser = parse_document(RcDom::default(), ParseOpts::default());
        let dom = parser.one(text);
        let mut info: (i32, i32, u32) = (0, 0, 0);
        let labels = Self::parse(&dom.document, &LabelOption::default(), &mut info);
        self.labels.replace(labels.clone());
        self.size.replace(Some(Self::measure_labels(&labels)));
        for label in &labels {
            self.add_child(label.clone(), AddChildOption::default());
        }
    }

    fn parse(handle: &Handle, option: &LabelOption, info: &mut (i32, i32, u32)) -> Vec<Rc<Node<OneLineLabel>>> {
        let (mut labels, next_option) = match handle.data {
            NodeData::Text { ref contents } => {
                let text = contents.borrow().to_string()
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&amp;", "&");
                let texts: Vec<&str> = text.split("\n").collect();
                (Self::parse_text(texts, option, info), option.clone())
            },
            NodeData::Element { ref name, ref attrs, .. } => {
                let next_option = Self::parse_element(option, name.local.to_string(), attrs.borrow().as_ref());
                (Vec::new(), next_option)
            },
            _ => { (Vec::new(), option.clone()) }
        };
        for child in handle.children.borrow().iter() {
            let mut c = Self::parse(&child, &next_option, info);
            labels.append(&mut c);
        }
        labels
    }

    fn parse_text(texts: Vec<&str>, option: &LabelOption, info: &mut (i32, i32, u32)) -> Vec<Rc<Node<OneLineLabel>>> {
        let mut result: Vec<Rc<Node<OneLineLabel>>> = Vec::new();
        for i in 0..texts.len() {
            let t = texts.get(i).unwrap().to_string();
            let label = OneLineLabel::create(t.as_str(), option);
            let size = label.get_size();
            label.set_anchor_point(AnchorPoint::new(0.0, 0.0));
            if i == 0 {
                info.2 = max(info.2, size.height());
                label.set_position(&Point::new(info.0, info.1));
                if &t != "" { info.0 += size.width() as i32 + 2; }
            } else {
                info.1 += info.2 as i32 + 2;
                label.set_position(Point::new(0, info.1));
                info.0 = if &t == "" { 0 } else { size.width() as i32 + 2 };
                info.2 = size.height();
            }
            result.push(label);
        }
        result
    }

    fn parse_element(option: &LabelOption, name: String, attrs: &Vec<Attribute>) -> LabelOption {
        let mut attr_map: HashMap<String, String> = HashMap::new();
        for attr in attrs {
            attr_map.insert(attr.name.local.to_string().to_lowercase(), attr.value.to_string());
        }
        LabelOption::parse(option, name.to_lowercase(), attr_map)
    }

    fn measure_labels(labels: &Vec<Rc<Node<OneLineLabel>>>) -> Size {
        let mut max_x: u32 = 0;
        let mut max_y: u32 = 0;
        for label in labels {
            let point = label.get_position();
            let size = label.get_size();
            let (current_x, current_y) = (point.x() as u32 + size.width(), point.y() as u32 + size.height());
            max_x = max(max_x, current_x);
            max_y = max(max_y, current_y);
        }
        Size::new(max_x, max_y)
    }

    fn clear_labels(&self) {
        {
            for label in self.labels.borrow().iter() {
                label.destroy();
            }
        }
        self.labels.replace(Vec::new());
    }

    fn updated(&self) {
        self.clear_labels();
        self.build();
        self.clear_cache();
    }

}

impl NodeDelegate for PrettyLabel {

    fn get_size(&self) -> Size {
        self.size.borrow().clone().unwrap()
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self, _parent: Rc<dyn NodeLike>) { }

    fn render(&self, _parent: Rc<dyn NodeLike>) { }

}

