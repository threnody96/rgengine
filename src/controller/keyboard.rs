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
        let map = if repeat { self.repeat_map.borrow() } else { self.pushed_map.borrow() };
        let r = map.get(vkey.to_string().as_str()).cloned();
        r.is_some() && r.unwrap()
    }

    pub fn keydown(&self, keycode: &KeyCode, repeat: bool) {
        match self.get_virtual_key(keycode) {
            None => { },
            Some(vkey) => {
                self.update_pushed_map(&vkey, !repeat);
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
