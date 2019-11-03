use std::cell::RefCell;
use std::rc::Rc;
use ::node::{ Node };
use ::node::label::{ Label, LabelOption };
use ::util::{ FuzzyArg };

pub struct LabelBuilder {
    option: RefCell<Option<LabelOption>>
}

impl LabelBuilder {

    pub fn new() -> Self {
        Self {
            option: RefCell::new(None)
        }
    }

    pub fn with_option<T>(self, option: T) -> Self
    where T: FuzzyArg<LabelOption>
    {
        self.option.replace(Some(option.take()));
        self
    }

    pub fn create<T>(self, text: T) -> Rc<Node<Label>>
    where T: FuzzyArg<String>
    {
        Label::create_with_option(text.take(), self.option.borrow().clone())
    }

}