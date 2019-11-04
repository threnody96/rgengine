use ::util::parameter::{ Point };

pub struct InputInfo {
    pub(crate) press_start: bool,
    pub(crate) pressing: bool,
    pub(crate) pressed: bool,
    pub(crate) mouse_position: Point,
    pub(crate) axis_position: Point
}

impl InputInfo {

    pub fn new() -> Self {
        Self {
            press_start: false,
            pressing: false,
            pressed: false,
            mouse_position: Point::new(0, 0),
            axis_position: Point::new(0, 0)
        }
    }

    pub fn is_press_start(&self) -> bool {
        self.press_start
    }

    pub fn is_pressing(&self) -> bool {
        self.pressing
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn get_mouse_position(&self) -> Point {
        self.mouse_position.clone()
    }

    pub fn get_axis_postiion(&self) -> Point {
        self.axis_position.clone()
    }

    pub(crate) fn update_press_state(&mut self, status: (bool, bool, bool)) {
        self.press_start = self.press_start || status.0;
        self.pressing = self.pressing || status.1;
        self.pressed = self.pressed || status.2;
    }

}