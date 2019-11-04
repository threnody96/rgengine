use ::util::{ director };
use ::util::parameter::{ Point, InputInfo };

pub fn is_press_start<A>(key: A) -> bool where A: Into<String> {
    get_input(key).is_press_start()
}

pub fn is_pressing<A>(key: A) -> bool where A: Into<String> {
    get_input(key).is_pressing()
}

pub fn is_pressed<A>(key: A) -> bool where A: Into<String> {
    get_input(key).is_pressed()
}

pub fn is_click<A>(key: A) -> Option<Point> where A: Into<String> {
    let info = get_input(key);
    if !info.is_pressed() { return None; }
    Some(info.get_mouse_position())
}

pub fn get_mouse_position() -> Point {
    director(|d| d.get_mouse_position())
}

pub fn get_input<A>(key: A) -> InputInfo where A: Into<String> {
    director(|d| d.get_input_info(key))
}

