use std::collections::HashMap;
use std::cell::RefCell;
use ggez::event::{ Button };

pub struct Gamepad {
    buttonmap: HashMap<Button, String>,
    pushed_map: RefCell<HashMap<String, bool>>,
    repeat_map: RefCell<HashMap<String, bool>>,
}

impl Gamepad {

    pub fn new<B>(buttonmap: HashMap<Button, B>) -> Self
    where B: ToString
    {
        let mut bmap: HashMap<Button, String> = HashMap::new();
        for (key, val) in &buttonmap { bmap.insert(key.clone(), val.to_string()); }
        Self {
            buttonmap: bmap,
            pushed_map: RefCell::new(HashMap::new()),
            repeat_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_key<B>(&self, bkey: &B, repeat: bool) -> bool
    where B: ToString
    {
        let v = bkey.to_string();
        let pushed = if repeat { self.is_repeat(&v) } else { self.is_pushed(&v) };
        self.update_pushed_map(&v, false);
        pushed
    }

    pub fn keydown(&self, button: &Button) {
        match self.get_virtual_button(button) {
            None => { },
            Some(vbutton) => {
                self.update_pushed_map(&vbutton, !self.is_repeat(&vbutton) && !self.is_pushed(&vbutton));
                self.update_repeat_map(&vbutton, true);
            }
        }
    }

    pub fn keyup(&self, button: &Button) {
        match self.get_virtual_button(button) {
            None => { },
            Some(vbutton) => {
                self.update_pushed_map(&vbutton, false);
                self.update_repeat_map(&vbutton, false);
            }
        }
    }

    fn is_pushed(&self, vbutton: &str) -> bool {
        let map = self.pushed_map.borrow();
        let p = map.get(vbutton).cloned();
        p.is_some() && p.unwrap()
    }

    fn is_repeat(&self, vbutton: &str) -> bool {
        let map = self.repeat_map.borrow();
        let p = map.get(vbutton).cloned();
        p.is_some() && p.unwrap()
    }

    fn update_pushed_map(&self, vbutton: &str, state: bool) {
        let mut map = self.pushed_map.borrow_mut();
        map.insert(vbutton.to_owned(), state);
    }

    fn update_repeat_map(&self, vbutton: &str, state: bool) {
        let mut map = self.repeat_map.borrow_mut();
        map.insert(vbutton.to_owned(), state);
    }

    fn get_virtual_button(&self, button: &Button) -> Option<String> {
        self.buttonmap.get(button).cloned()
    }

}
