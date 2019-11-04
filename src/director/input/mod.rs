use std::cell::RefCell;
use std::collections::HashMap;
use ::util::parameter::{ Point };
use sdl2::{ EventPump };
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct InputDirector {
    quit: bool,
    mouse: Point,
    keys: HashMap<Keycode, bool>
}

impl InputDirector {

    pub fn new() -> Self {
        Self {
            quit: false,
            mouse: Point::new(0, 0),
            keys: HashMap::new()
        }
    }

    pub fn update_state(&mut self, events: &mut EventPump) {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit = true;
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(k) = &keycode {
                        self.keys.insert(k.clone(), true);
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    if let Some(k) = &keycode {
                        self.keys.insert(k.clone(), false);
                    }
                },
                Event::MouseMotion { mousestate, .. } => {
                    self.mouse = Point::new(mousestate.x(), mousestate.y());
                },
                _ => {}
            }
        }
    }

    fn is_key_pressed(&self, keycode: &Keycode) -> bool {
        self.keys.get(keycode).cloned().unwrap_or(false)
    }

    pub fn is_quit(&self) -> bool {
        self.quit
    }

}