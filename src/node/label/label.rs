use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use ::node::{ Node, NodeLike, NodeDelegate, AddChildOption };
use ::node::label::{ LabelOption, OneLineLabel };
use ::util::{ FuzzyArg, NoOption };
use ::util::parameter::{ Point, Size, AnchorPoint };
pub use sdl2::pixels::{ Color };

pub struct Label {
    size: RefCell<Option<Size>>,
    text: RefCell<String>,
    labels: RefCell<Vec<Rc<Node<OneLineLabel>>>>,
    option: RefCell<LabelOption>,
}

impl Label {

    pub fn create<A, B>(text: A, option: B) -> Rc<Node<Self>>
    where
        A: FuzzyArg<String>,
        B: FuzzyArg<LabelOption>
    {
        let t = text.take();
        let o = option.take();
        let n = Node::create(|| {
            Self {
                size: RefCell::new(None),
                text: RefCell::new(t.clone()),
                labels: RefCell::new(Vec::new()),
                option: RefCell::new(o.clone())
            }
        });
        n.build();
        n
    }

    fn build(&self) {
        let text = self.text.borrow().clone();
        let texts: Vec<&str> = text.split("\n").collect();
        let option = self.option.borrow().clone();
        let mut prev_height: u32 = 0;
        let mut labels: Vec<Rc<Node<OneLineLabel>>> = Vec::new();
        let mut max_width: u32 = 0;
        for t in texts {
            let label = OneLineLabel::create(t, &option);
            label.set_anchor_point(AnchorPoint::new(0.0, 0.0));
            label.set_position(Point::new(0, prev_height as i32));
            let current_size = label.get_size();
            prev_height += current_size.height() + 2;
            max_width = max(current_size.width(), max_width);
            labels.push(label);
        }
        self.size.replace(Some(Size::new(max_width, prev_height)));
        self.labels.replace(labels.clone());
        for label in labels {
            self.add_child(label, AddChildOption::default())
        }
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

    pub fn set_text<A>(&self, text: A)
    where A: FuzzyArg<String>
    {
        self.text.replace(text.take());
        self.updated();
    }

    pub fn set_point<A>(&self, point: A)
    where A: FuzzyArg<u16>
    {
        {
            let mut option = self.option.borrow_mut();
            option.point = point.take();
        }
        self.updated();
    }

}

impl NodeDelegate for Label {

    fn get_size(&self) -> Size {
        self.size.borrow().clone().unwrap()
    }

    fn use_cache(&self) -> bool {
        true
    }

    fn update(&self, _parent: Rc<dyn NodeLike>) { }

    fn render(&self, _parent: Rc<dyn NodeLike>) {
    }

}
