use std::collections::HashMap;
use std::cell::RefCell;
use ggez::input::keyboard::KeyCode;

pub struct Keyboard {
    keymap: HashMap<KeyCode, String>,
    pushed_map: RefCell<HashMap<String, bool>>,
    repeat_map: RefCell<HashMap<String, bool>>,
}

impl Keyboard {

    pub fn new<K>(keymap: HashMap<KeyCode, K>) -> Self
    where K: ToString
    {
        let mut kmap: HashMap<KeyCode, String> = HashMap::new();
        for (key, val) in &keymap { kmap.insert(key.clone(), val.to_string()); }
        Self {
            keymap: kmap,
            pushed_map: RefCell::new(HashMap::new()),
            repeat_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_key<K>(&self, vkey: &K, repeat: bool) -> bool
    where K: ToString
    {
        let v = vkey.to_string();
        let pushed = if repeat { self.is_repeat(&v) } else { self.is_pushed(&v) };
        self.update_pushed_map(&v, false);
        pushed
    }

    pub fn keydown(&self, keycode: &KeyCode) {
        match self.get_virtual_key(keycode) {
            None => { },
            Some(vkey) => {
                self.update_pushed_map(&vkey, !self.is_repeat(&vkey) && !self.is_pushed(&vkey));
                self.update_repeat_map(&vkey, true);
            }
        }
    }

    pub fn keyup(&self, keycode: &KeyCode) {
        match self.get_virtual_key(keycode) {
            None => { },
            Some(vkey) => {
                self.update_pushed_map(&vkey, false);
                self.update_repeat_map(&vkey, false);
            }
        }
    }

    fn is_pushed(&self, vkey: &str) -> bool {
        let map = self.pushed_map.borrow();
        let p = map.get(vkey).cloned();
        p.is_some() && p.unwrap()
    }

    fn is_repeat(&self, vkey: &str) -> bool {
        let map = self.repeat_map.borrow();
        let p = map.get(vkey).cloned();
        p.is_some() && p.unwrap()
    }

    fn update_pushed_map(&self, vkey: &str, state: bool) {
        let mut map = self.pushed_map.borrow_mut();
        map.insert(vkey.to_owned(), state);
    }

    fn update_repeat_map(&self, vkey: &str, state: bool) {
        let mut map = self.repeat_map.borrow_mut();
        map.insert(vkey.to_owned(), state);
    }

    fn get_virtual_key(&self, keycode: &KeyCode) -> Option<String> {
        self.keymap.get(keycode).cloned()
    }

}
