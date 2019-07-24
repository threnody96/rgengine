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
        let pushed = {
            let map = if repeat { self.repeat_map.borrow() } else { self.pushed_map.borrow() };
            let r = map.get(bkey.to_string().as_str()).cloned();
            r.is_some() && r.unwrap()
        };
        if pushed { self.pushed_map.borrow_mut().insert(bkey.to_string(), false); }
        pushed
    }

    pub fn keydown(&self, button: &Button) {
        match self.get_virtual_button(button) {
            None => { },
            Some(vbutton) => {
                self.update_pushed_map(&vbutton, true);
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
