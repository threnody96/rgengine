use std::rc::Rc;
use ::node::scene::transition::{ SceneTransition, SceneTransitionDelegate, TransitionStatus };
use ::util::{ NoOption };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };
use sdl2::render::{ Texture, BlendMode };

pub struct TransitionNone {

}

impl TransitionNone {

    pub fn create() -> Rc<SceneTransition> {
        SceneTransition::create(|| {
            Rc::new(Self {})
        }, None, None)
    }

}

impl SceneTransitionDelegate for TransitionNone {

    fn canvas_blend_mode(&self) -> BlendMode {
        BlendMode::None
    }

    fn render<'a>(&self, canvas: &mut Canvas<Window>, scene: &mut Texture<'a>, _prev_scene: &mut Texture<'a>, _progress: f32) -> Option<TransitionStatus> {
        canvas.copy(scene, None, None).unwrap();
        None
    }

}

impl From<NoOption> for Rc<SceneTransition> {

    fn from(_: NoOption) -> Rc<SceneTransition> {
        TransitionNone::create()
    }

}
