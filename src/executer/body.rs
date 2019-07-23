use std::rc::Rc;
use ::controller::Input;

pub trait GameBody {

    fn input(&self) -> Rc<Input>;

}
