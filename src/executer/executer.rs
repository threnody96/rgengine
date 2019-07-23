use ggez::{ GameResult, Context, ContextBuilder };
use ggez::graphics::{ BLACK, clear, present };
use ggez::event::{ EventHandler, run, KeyCode, KeyMods, Button };
use ggez::input::gamepad::GamepadId;
use ::executer::GameBody;
use ::component::Component;

pub struct GameExecuter<T> where T: GameBody + Component {
    body: T
}

impl <T> GameExecuter<T> where T: GameBody + Component {

    pub fn new(body: T) -> Self {
        Self { body: body }
    }

    pub fn run(&mut self, game_id: &'static str, author: &'static str) -> GameResult {
        let cb = ContextBuilder::new(game_id, author);
        let (ctx, events_loop) = &mut cb.build()?;
        run(ctx, events_loop, self)
    }

}

impl <T> EventHandler for GameExecuter<T> where T: GameBody + Component {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.body.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BLACK);
        let result = self.body.draw(ctx)?;
        present(ctx)?;
        Ok(result)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.body.input().update_keyboard_keydown(&keycode, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.body.input().update_keyboard_keyup(&keycode);
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.body.input().update_gamepad_keydown(&_id, &_btn);
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.body.input().update_gamepad_keyup(&_id, &_btn);
    }

}
