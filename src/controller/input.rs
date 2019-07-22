use std::collections::HashMap;
use ::controller::{ Keyboard, Gamepad };
use ggez::input::gamepad::GamepadId;
use ggez::input::keyboard::KeyCode;
use ggez::event::{ Button };

pub struct Input {
    keyboard: Keyboard,
    gamepad: HashMap<GamepadId, Gamepad>,
}

impl Input {

    pub fn new<K, B>(keymap: HashMap<KeyCode, K>, buttonmap: HashMap<GamepadId, HashMap<Button, B>>) -> Self
    where K: ToString, B: ToString
    {
        let mut map: HashMap<GamepadId, Gamepad> = HashMap::new();
        for (key, val) in buttonmap { map.insert(key, Gamepad::new(val)); }
        Self {
            keyboard: Keyboard::new(keymap),
            gamepad: map,
        }
    }

    pub fn get_key<B>(&self, key: &B, repeat: bool) -> bool
    where B: ToString
    {
        if self.keyboard.get_key(key, repeat) { return true; }
        for (_id, gamepad) in &self.gamepad {
            if gamepad.get_key(key, repeat) { return true; }
        }
        false
    }

    pub fn update_keyboard_keydown(&self, keycode: &KeyCode, repeat: bool) {
        self.keyboard.keydown(keycode, repeat);
    }

    pub fn update_keyboard_keyup(&self, keycode: &KeyCode) {
        self.keyboard.keyup(keycode);
    }

    pub fn update_gamepad_keydown(&self, id: &GamepadId, button: &Button) {
        let pad = self.gamepad.get(id);
        if pad.is_none() { return; }
        pad.unwrap().keydown(button);
    }

    pub fn update_gamepad_keyup(&self, id: &GamepadId, button: &Button) {
        let pad = self.gamepad.get(id);
        if pad.is_none() { return; }
        pad.unwrap().keyup(button);
    }

}
