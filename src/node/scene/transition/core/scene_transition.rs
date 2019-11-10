use std::rc::Rc;
use std::cell::RefCell;
use ::node::scene::transition::{ SceneTransitionDelegate, TransitionStatus };
use ::util::{with_context};
use ::util::parameter::{ Size };
use ::util::easing::{ EasingFunction };
use sdl2::render::{ Texture, BlendMode };
use sdl2::pixels::{ PixelFormatEnum, Color };
use time::{ Tm };

pub struct SceneTransition {
    delegate: Rc<dyn SceneTransitionDelegate>,
    status: RefCell<TransitionStatus>,
    start: RefCell<Option<Tm>>,
    duration: Option<f32>,
    easing: Option<Rc<dyn EasingFunction>>
}

impl SceneTransition {

    pub fn create<T>(callback: T, duration: Option<f32>, easing: Option<Rc<dyn EasingFunction>>) -> Rc<SceneTransition> where T: Fn() -> Rc<dyn SceneTransitionDelegate> {
        Rc::new(SceneTransition {
            delegate: callback(),
            status: RefCell::new(TransitionStatus::Wait),
            start: RefCell::new(None),
            duration: duration,
            easing: easing
        })
    }

    pub fn render<'a>(&self, mut scene: Texture<'a>, mut prev_scene: Texture<'a>) -> Texture<'a> {
        if self.initialize() {
            let query = scene.query();
            let mut canvas = with_context(|c| c.create_sub_canvas(Size::new(query.width, query.height)));
            canvas.set_blend_mode(self.delegate.canvas_blend_mode());
            let progress = self.generate_progress();
            let mut status: Option<TransitionStatus> = None;
            with_context(|c| &mut c.canvas).with_texture_canvas(&mut canvas, |c| {
                status = self.delegate.render(c, &mut scene, &mut prev_scene, self.generate_progress());
            });
            if let Some(s) = status.clone() {
                self.status.replace(s);
            } else if progress == 1.0 {
                self.status.replace(TransitionStatus::Finish);
            }
            return canvas;
        }
        scene
    }

    fn generate_progress(&self) -> f32 {
        if let Some(duration) = self.duration {
            if duration == 0.0 { return 1.0; }
            let start = self.start.borrow().clone().unwrap();
            let d = (time::now() - start).num_microseconds().unwrap();
            let progress = d as f32 / (duration * 1_000_000.0);
            if progress > 1.0 { return 1.0; }
            if let Some(easing) = self.easing.clone() { return easing.ease(progress); }
            progress
        } else {
            1.0
        }
    }

    pub fn get_status(&self) -> TransitionStatus {
        self.status.borrow().clone()
    }

    fn initialize(&self) -> bool {
        let status = self.status.borrow().clone();
        match status {
            TransitionStatus::Wait => {
                self.start.replace(Some(time::now()));
                self.status.replace(TransitionStatus::Processing);
                true
            },
            TransitionStatus::Processing => {
                true
            },
            TransitionStatus::Finish => {
                false
            }
        }
    }

}

