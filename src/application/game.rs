use std::rc::Rc;
use ::application::{ AppDelegate };
use ::node::{ SceneLike };
use ::util::{ Point, Size, must, director, context };
use ggez::{ Context, GameResult };
use ggez::event::{ EventHandler };
use ggez::timer::{ check_update_time, fps };
use ggez::graphics::{ Canvas, DrawParam, Text, TextFragment, BLACK, WHITE, Font, Scale, draw, clear, present, set_canvas };
use ggez::conf::NumSamples;

pub struct Game {
    canvas: Canvas,
    delegate: Rc<dyn AppDelegate>
}

impl Game {

    pub fn new(delegate: Rc<dyn AppDelegate>) -> Self {
        Self {
            canvas: context(|ctx| must(Canvas::new(ctx, 400, 300, NumSamples::One))),
            delegate: delegate
        }
    }

    fn get_scene(&self) -> Rc<dyn SceneLike> {
        director(|d| d.get_scene())
    }

    fn draw_debug_message(&self) -> GameResult<()> {
        if director(|d| !d.get_display_stats()) { return Ok(()); }
        context(|ctx| {
            let fps_display = Text::new(TextFragment {
                text: format!("FPS: {}", fps(ctx).round() as u32),
                color: Some(WHITE),
                font: Some(Font::default()),
                scale: Some(Scale::uniform(15.0)),
                ..Default::default()
            });
            draw(
                ctx,
                &fps_display,
                (Point { x: 0.0, y: 0.0}, WHITE)
            )
        })?;
        Ok(())
    }

}

impl EventHandler for Game {

    fn update(&mut self) -> GameResult<()> {
        while { context(|ctx| check_update_time(ctx, self.delegate.fps())) } {
            self.get_scene().update();
        }
        Ok(())
    }

    fn draw(&mut self) -> GameResult<()> {
        context(|ctx| {
            set_canvas(ctx, Some(&self.canvas));
            clear(ctx, BLACK);
        });
        self.get_scene().render();
        self.draw_debug_message();
        context(|ctx| {
            present(ctx);
            set_canvas(ctx, None);
            clear(ctx, BLACK);
            ggez::graphics::draw(ctx, &self.canvas, DrawParam {
                dest: Point { x: 0.0, y: 0.0 },
                ..Default::default()
            });
            present(ctx)
        })
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        director(|d| {
            d.set_visible_size(Size {
                width: width,
                height: height
            })
        });
    }

}

