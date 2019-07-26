use std::rc::Rc;
use ggez::{ GameResult, Context, ContextBuilder };
use ggez::graphics::{ BLACK, clear, present };
use ggez::event::{ EventHandler, run, KeyCode, KeyMods, Button };
use ggez::input::gamepad::GamepadId;
use ::component::Component;
use ::resource::Resource;

pub struct GameExecuter {
    scene: Box<dyn Component>,
    rsc: Rc<Resource>
}

impl GameExecuter {

    pub fn update_scene(&mut self, scene: Box<dyn Component>) {
        self.scene = scene;
    }

    pub fn run(game_id: &'static str, author: &'static str, rsc: Resource, scene: Box<dyn FnOnce(&mut Context, Rc<Resource>) -> Box<dyn Component>>) -> GameResult {
        let r = Rc::new(rsc);
        let cb = ContextBuilder::new(game_id, author);
        let (ctx, events_loop) = &mut cb.build()?;
        let mut executer = Self {
            scene: scene(ctx, r.clone()),
            rsc: r.clone()
        };
        run(ctx, events_loop, &mut executer)
    }

}

impl EventHandler for GameExecuter {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        (*self.scene).update(ctx, self.rsc.clone())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BLACK);
        let result = (*self.scene).draw(ctx, self.rsc.clone())?;
        present(ctx)?;
        Ok(result)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.rsc.input.update_keyboard_keydown(&keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.rsc.input.update_keyboard_keyup(&keycode);
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.rsc.input.update_gamepad_keydown(&_id, &_btn);
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.rsc.input.update_gamepad_keyup(&_id, &_btn);
    }

}
