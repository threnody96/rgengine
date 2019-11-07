use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{ min, max };
use ::node::{ Node, NodeLike, NodeDelegate, AddChildOption };
use ::node::label::{ LabelOption, OneLineLabel };
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
        A: Into<String>,
        B: Into<LabelOption>
    {
        let t = text.into();
        let o = option.into();
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
        let border_width = self.generate_border_width();
        for t in texts {
            let label = OneLineLabel::create(t, &option);
            label.set_anchor_point(AnchorPoint::new(0.0, 0.0));
            label.set_position(Point::new(0, prev_height as i32));
            if let Some(border) = &option.border {
                for offset in self.generate_border_points(border_width) {
                    let l = OneLineLabel::create(t, LabelOption {
                        color: border.clone(),
                        border: None,
                        ..option.clone()
                    });
                    l.set_anchor_point(AnchorPoint::new(0.0, 0.0));
                    l.set_position(Point::new(offset.0, prev_height as i32 + offset.1));
                    labels.push(l);
                }
            }
            let current_size = label.get_size();
            prev_height += current_size.height() + 2 + border_width as u32 * 2;
            max_width = max(current_size.width() + border_width as u32 * 2, max_width);
            labels.push(label);
        }
        self.size.replace(Some(Size::new(max_width, prev_height)));
        self.labels.replace(labels.clone());
        for label in labels {
            self.add_child(label, AddChildOption::default())
        }
    }

    fn generate_border_width(&self) -> i32 {
        let option = self.option.borrow().clone();
        match &option.border {
            None => { 0 },
            Some(_) => {
                min(3, max((option.point / 20) as i32, 1))
            }
        }
    }

    fn generate_border_points(&self, border_width: i32) -> Vec<(i32, i32)> {
        vec!(
            (-border_width, -border_width),
            (-border_width, 0),
            (-border_width, border_width),
            (0, -border_width),
            (0, border_width),
            (border_width, -border_width),
            (border_width, 0),
            (border_width, border_width),
        )
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
    where A: Into<String>
    {
        self.text.replace(text.into());
        self.updated();
    }

    pub fn set_point<A>(&self, point: A)
    where A: Into<u16>
    {
        {
            let mut option = self.option.borrow_mut();
            option.point = point.into();
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

    fn update(&self) { }

    fn render(&self) {
    }

}
