use std::cell::RefCell;
use std::rc::Rc;
use ::application::{ AppDelegate };
use ::util::{ Size, Point };
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{run, EventHandler};

pub struct Game {
    delegate: Rc<dyn AppDelegate>
}

impl Game {

    pub fn new(ctx: &mut Context, delegate: Rc<dyn AppDelegate>) -> Self {
        Self {
            delegate: delegate
        }
    }

}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

pub struct Application {
    delegate: RefCell<Option<Rc<dyn AppDelegate>>>,
    visible_size: RefCell<Option<Size>>
}

impl Application {

    pub fn new() -> Self {
        Self {
            delegate: RefCell::new(None),
            visible_size: RefCell::new(None)
        }
    }

    pub fn get_visible_size(&self) -> Size {
        let visible_size = self.visible_size.borrow();
        if visible_size.is_none() { panic!("ゲームが実行されていません"); }
        visible_size.unwrap().clone()
    }

    pub fn set_visible_size(&self, size: Size) {
        self.visible_size.replace(Some(size));
    }

    pub fn get_visible_origin(&self) -> Point {
        Point { x: 0.0f32, y: 0.0f32 }
    }

    pub fn run(&self, delegate: Rc<dyn AppDelegate>) {
        self.delegate.replace(Some(delegate.clone()));
        self.set_visible_size(Size {
            width: delegate.window_mode().width,
            height: delegate.window_mode().height
        });
        let (mut ctx, mut event_loop) = ContextBuilder::new("game_id", &delegate.author())
            .window_mode(delegate.window_mode())
            .window_setup(delegate.window_setup())
            .build()
            .expect("aieee, could not create ggez context!");

        let mut game = Game::new(&mut ctx, delegate);

        match run(&mut ctx, &mut event_loop, &mut game) {
            Ok(_) => { },
            Err(e) => { panic!(format!("初期化に失敗しました: {}", e)); }
        }
    }

}