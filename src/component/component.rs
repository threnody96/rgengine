use ggez::{ GameResult, Context, ContextBuilder };
use ggez::graphics::{ BLACK, clear, present };
use ggez::event::{ EventHandler, run, KeyCode, KeyMods, Button };
use ggez::input::gamepad::GamepadId;
use ::component::GameBody;
use ::controller::Input;

pub struct GameComponent {
    input: Input,
    body: Box<dyn GameBody>,
}

impl GameComponent {

    pub fn new(body: Box<dyn GameBody>, input: Input) -> Self {
        Self {
            input: input,
            body: body,
        }
    }

    pub fn run(&mut self, game_id: &'static str, author: &'static str) -> GameResult {
        let cb = ContextBuilder::new(game_id, author);
        let (ctx, events_loop) = &mut cb.build()?;
        run(ctx, events_loop, self)
    }

}

impl EventHandler for GameComponent {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        (*self.body).update(ctx, &self.input)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BLACK);
        let result = (*self.body).draw(ctx)?;
        present(ctx)?;
        Ok(result)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input.update_keyboard_keydown(&keycode, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input.update_keyboard_keyup(&keycode);
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.input.update_gamepad_keydown(&_id, &_btn);
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.input.update_gamepad_keyup(&_id, &_btn);
    }

}
