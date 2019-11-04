use std::cell::RefCell;
use std::collections::HashMap;
use ::util::parameter::{ Point, InputCodeMap, InputCode, InputInfo };
use sdl2::{ EventPump };
use sdl2::keyboard::Keycode;
use sdl2::mouse::{ MouseButton, MouseWheelDirection };
use sdl2::event::Event;
use sdl2::joystick::HatState;
use std::convert::TryInto;

#[derive(Clone)]
struct InputState {
    mouse_pointer: Point,
    mouses: HashMap<MouseButton, bool>,
    mousewheels: HashMap<MouseWheelDirection, bool>,
    joystikcs: HashMap<i32, JoystickState>,
    keys: HashMap<Keycode, bool>,
}

impl InputState {

    fn new() -> Self {
        Self {
            mouse_pointer: Point::new(-1, -1),
            mouses: HashMap::new(),
            mousewheels: HashMap::new(),
            joystikcs: HashMap::new(),
            keys: HashMap::new()
        }
    }

    fn init_joysticks(&mut self, index: i32) {
        if self.joystikcs.get(&index).is_none() {
            self.joystikcs.insert(index, JoystickState::new());
        }
    }

    fn reset_joystick(&mut self) {
        for (_, s) in &mut self.joystikcs {
            s.reset();
        }
    }

}

#[derive(Clone)]
struct JoystickState {
    buttons: HashMap<u8, bool>,
    hats: HashMap<HatState, bool>,
    sticks: Point
}

impl JoystickState {

    fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            hats: HashMap::new(),
            sticks: Point::new(0, 0)
        }
    }

    fn reset(&mut self) {
        self.sticks = Point::new(0, 0);
        self.hats = HashMap::new()
    }

}

pub struct InputDirector {
    quit: bool,
    input_code_map: InputCodeMap,
    prev_state: InputState,
    state: InputState
}

impl InputDirector {

    pub fn new() -> Self {
        Self {
            quit: false,
            input_code_map: InputCodeMap::new(),
            prev_state: InputState::new(),
            state: InputState::new()
        }
    }

    pub fn get_input_info<A>(&self, key: A) -> InputInfo
    where A: Into<String>
    {
        let mut info = InputInfo::new();
        for k in self.input_code_map.convert_key(key) {
            let keycode: Result<Keycode, String> = k.clone().try_into();
            if let Ok(k) = keycode {
                info.update_press_state(self.get_keyboard_state(&k));
                continue;
            }
            let mouse: Result<MouseButton, String> = k.clone().try_into();
            if let Ok(k) = mouse {
                let status = self.get_mouse_button_state(&k);
                info.update_press_state(status.clone());
                if status.0 || status.1 || status.2 { info.mouse_position = self.get_mouse_pointer(); }
                continue;
            }
            match k.clone() {
                InputCode::MouseWheelUp => {
                    let status = self.get_mouse_wheel_state(MouseWheelDirection::Normal);
                    info.update_press_state(status.clone());
                    if status.0 || status.1 || status.2 { info.mouse_position = self.get_mouse_pointer(); }
                },
                InputCode::MouseWheelDown => {
                    let status = self.get_mouse_wheel_state(MouseWheelDirection::Flipped);
                    info.update_press_state(status.clone());
                    if status.0 || status.1 || status.2 { info.mouse_position = self.get_mouse_pointer(); }
                },
                InputCode::JoystickButton { index, button } => {
                    info.update_press_state(self.get_joystick_button_state(index, button));
                },
                InputCode::JoystickHatUp { index } => {
                    info.update_press_state(self.get_joystick_hat_state(index, HatState::Up));
                },
                InputCode::JoystickHatDown { index } => {
                    info.update_press_state(self.get_joystick_hat_state(index, HatState::Down));
                },
                InputCode::JoystickHatLeft { index} => {
                    info.update_press_state(self.get_joystick_hat_state(index, HatState::Left));
                },
                InputCode::JoystickHatRight { index } => {
                    info.update_press_state(self.get_joystick_hat_state(index, HatState::Right));
                },
                InputCode::JoystickAxis { index } => {
                    info.axis_position = self.get_joystick_axis(index);
                },
                _ => {}
            }
        }
        info
    }

    pub fn insert_key_code<A>(&mut self, key: A, code: InputCode)
    where A: Into<String>
    {
        self.input_code_map.insert(key, code);
    }

    pub fn reset_key_code<A>(&mut self, key: Option<A>)
        where A: Into<String>
    {
        self.input_code_map.reset(key);
    }

    pub fn update_state(&mut self, events: &mut EventPump) {
        self.prev_state = self.state.clone();
        self.state.mousewheels = HashMap::new();
        self.state.reset_joystick();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit = true;
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(k) = &keycode {
                        self.state.keys.insert(k.clone(), true);
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    if let Some(k) = &keycode {
                        self.state.keys.insert(k.clone(), false);
                    }
                },
                Event::MouseMotion { mousestate, .. } => {
                    self.state.mouse_pointer = Point::new(mousestate.x(), mousestate.y());
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.state.mouses.insert(mouse_btn.clone(), true);
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.state.mouses.insert(mouse_btn.clone(), false);
                },
                Event::MouseWheel { direction, .. } => {
                    self.state.mousewheels.insert(direction, true);
                },
                Event::JoyHatMotion { which, state, .. } => {
                    self.state.init_joysticks(which);
                    let j = self.state.joystikcs.get_mut(&which).unwrap();
                    j.hats.insert(state, true);
                },
                Event::JoyButtonDown { which, button_idx, .. } => {
                    self.state.init_joysticks(which);
                    let j = self.state.joystikcs.get_mut(&which).unwrap();
                    j.buttons.insert(button_idx, true);
                },
                Event::JoyButtonUp { which, button_idx, .. } => {
                    self.state.init_joysticks(which);
                    let j = self.state.joystikcs.get_mut(&which).unwrap();
                    j.buttons.insert(button_idx, false);
                },
                Event::JoyAxisMotion { which, axis_idx, value, .. } => {
                    self.state.init_joysticks(which);
                    let j = self.state.joystikcs.get_mut(&which).unwrap();
                    if axis_idx == 0 {
                        j.sticks = Point::new(value as i32, j.sticks.y());
                    }
                    if axis_idx == 1 {
                        j.sticks = Point::new(j.sticks.x(), value as i32);
                    }
                },
                Event::JoyDeviceRemoved { which, .. } => {
                    self.state.joystikcs.remove(&which);
                },
                _ => {}
            }
        }
    }

    fn get_mouse_pointer(&self) -> Point {
        self.state.mouse_pointer.clone()
    }

    fn get_mouse_button_state(&self, button: &MouseButton) -> (bool, bool, bool) {
        let prev_state = self.prev_state.mouses.get(button).cloned().unwrap_or(false);
        let state = self.state.mouses.get(button).cloned().unwrap_or(false);
        (!prev_state && state, state, prev_state && !state)
    }

    fn get_mouse_wheel_state(&self, direction: MouseWheelDirection) -> (bool, bool, bool) {
        let prev_state = self.prev_state.mousewheels.get(&direction).cloned().unwrap_or(false);
        let state = self.state.mousewheels.get(&direction).cloned().unwrap_or(false);
        (!prev_state && state, state, prev_state && !state)
    }

    fn get_keyboard_state(&self, key: &Keycode) -> (bool, bool, bool) {
        let prev_state = self.prev_state.keys.get(key).cloned().unwrap_or(false);
        let state = self.state.keys.get(key).cloned().unwrap_or(false);
        (!prev_state && state, state, prev_state && !state)
    }

    fn get_joystick_state(&self, index: i32) -> (Option<&JoystickState>, Option<&JoystickState>) {
        let prev_state = self.prev_state.joystikcs.get(&index);
        let state = self.state.joystikcs.get(&index);
        (prev_state, state)
    }

    fn get_joystick_hat_state(&self, index: i32, hat: HatState) -> (bool, bool, bool) {
        let (prev_state, state) = self.get_joystick_state(index);
        let p = prev_state.map(|e| e.hats.get(&hat).cloned().unwrap_or(false)).unwrap_or(false);
        let s = state.map(|e| e.hats.get(&hat).cloned().unwrap_or(false)).unwrap_or(false);
        (!p && s, s, p && !s)
    }

    fn get_joystick_button_state(&self, index: i32, button: u8) -> (bool, bool, bool) {
        let (prev_state, state) = self.get_joystick_state(index);
        let p = prev_state.map(|e| e.buttons.get(&button).cloned().unwrap_or(false)).unwrap_or(false);
        let s = state.map(|e| e.buttons.get(&button).cloned().unwrap_or(false)).unwrap_or(false);
        (!p && s, s, p && !s)
    }

    fn get_joystick_axis(&self, index: i32) -> Point {
        if let Some(s) = self.state.joystikcs.get(&index) {
            return s.sticks.clone();
        }
        JoystickState::new().sticks.clone()
    }

    pub fn is_quit(&self) -> bool {
        self.quit
    }

    pub fn reset_is_quit(&mut self) {
        self.quit = false;
    }

}