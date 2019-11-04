use std::cell::RefCell;
use std::collections::HashMap;
use ::util::parameter::{ Point };
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
    prev_state: InputState,
    state: InputState
}

impl InputDirector {

    pub fn new() -> Self {
        Self {
            quit: false,
            prev_state: InputState::new(),
            state: InputState::new()
        }
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

    fn get_mouse_button_state(&self, button: &MouseButton) -> (bool, bool) {
        let prev_state = self.prev_state.mouses.get(button).cloned().unwrap_or(false);
        let state = self.state.mouses.get(button).cloned().unwrap_or(false);
        (prev_state, state)
    }

    fn is_mouse_button_press_start(&self, button: MouseButton) -> Option<Point> {
        let (prev_state, state) = self.get_mouse_button_state(&button);
        if !prev_state && state { return Some(self.get_mouse_pointer()); }
        None
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> Option<Point> {
        let (prev_state, state) = self.get_mouse_button_state(&button);
        if prev_state && !state { return Some(self.get_mouse_pointer()); }
        None
    }

    fn is_mouse_button_pressing(&self, button: MouseButton) -> Option<Point> {
        let (_, state) = self.get_mouse_button_state(&button);
        if state { return Some(self.get_mouse_pointer()); }
        None
    }

    fn is_mouse_wheel_up(&self) -> bool {
        self.state.mousewheels.get(&MouseWheelDirection::Normal).cloned().unwrap_or(false)
    }

    fn is_mouse_wheel_down(&self) -> bool {
        self.state.mousewheels.get(&MouseWheelDirection::Flipped).cloned().unwrap_or(false)
    }

    fn get_keyboard_state(&self, key: &Keycode) -> (bool, bool) {
        let prev_state = self.prev_state.keys.get(key).cloned().unwrap_or(false);
        let state = self.state.keys.get(key).cloned().unwrap_or(false);
        (prev_state, state)
    }

    fn is_keyboard_press_start(&self, key: Keycode) -> bool {
        let (prev_state, state) = self.get_keyboard_state(&key);
        !prev_state && state
    }

    fn is_keyboard_pressed(&self, key: Keycode) -> bool {
        let (prev_state, state) = self.get_keyboard_state(&key);
        prev_state && !state
    }

    fn is_keyboard_pressing(&self, key: Keycode) -> bool {
        self.state.keys.get(&key).cloned().unwrap_or(false)
    }

    fn get_joystick_state(&self, index: i32) -> (Option<&JoystickState>, Option<&JoystickState>) {
        let prev_state = self.prev_state.joystikcs.get(&index);
        let state = self.state.joystikcs.get(&index);
        (prev_state, state)
    }

    fn get_joystick_hat_state(&self, index: i32, hat: &HatState) -> (bool, bool) {
        let (prev_state, state) = self.get_joystick_state(index);
        let p = prev_state.map(|e| e.hats.get(hat).cloned().unwrap_or(false)).unwrap_or(false);
        let s = state.map(|e| e.hats.get(hat).cloned().unwrap_or(false)).unwrap_or(false);
        (p, s)
    }

    fn is_joystick_hat_press_start(&self, index: i32, hat: HatState) -> bool {
        let (prev_state, state) = self.get_joystick_hat_state(index, &hat);
        !prev_state && state
    }

    fn is_joystick_hat_pressing(&self, index: i32, hat: HatState) -> bool {
        let (_, state) = self.get_joystick_hat_state(index, &hat);
        state
    }

    fn is_joystick_hat_pressed(&self, index: i32, hat: HatState) -> bool {
        let (prev_state, state) = self.get_joystick_hat_state(index, &hat);
        prev_state && !state
    }

    fn get_joystick_button_state(&self, index: i32, button: u8) -> (bool, bool) {
        let (prev_state, state) = self.get_joystick_state(index);
        let p = prev_state.map(|e| e.buttons.get(&button).cloned().unwrap_or(false)).unwrap_or(false);
        let s = state.map(|e| e.buttons.get(&button).cloned().unwrap_or(false)).unwrap_or(false);
        (p, s)
    }

    fn is_joystick_button_press_start(&self, index: i32, button: u8) -> bool {
        let (prev_state, state) = self.get_joystick_button_state(index, button);
        !prev_state && state
    }

    fn is_joystick_button_pressing(&self, index: i32, button: u8) -> bool {
        let (_, state) = self.get_joystick_button_state(index, button);
        state
    }

    fn is_joystick_button_pressed(&self, index: i32, button: u8) -> bool {
        let (prev_state, state) = self.get_joystick_button_state(index, button);
        prev_state && !state
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

}