use std::rc::Rc;
use ::node::scene::transition::{ SceneTransition, SceneTransitionDelegate, TransitionStatus };
use ::util::easing::{ EasingFunction };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };
use sdl2::render::{ Texture, BlendMode };

pub struct TransitionFade {

}

impl TransitionFade {

    pub fn create(duration: f32, easing: Option<Rc<dyn EasingFunction>>) -> Rc<SceneTransition> {
        SceneTransition::create(Rc::new(Self {}), Some(duration), easing)
    }

}

impl SceneTransitionDelegate for TransitionFade {

    fn canvas_blend_mode(&self) -> BlendMode {
        BlendMode::Blend
    }

    fn render<'a>(&self, canvas: &mut Canvas<Window>, scene: &mut Texture<'a>, prev_scene: &mut Texture<'a>, progress: f32) -> Option<TransitionStatus> {
        if progress < 0.5 {
            let alpha = ((1.0 - progress * 2.0) * 255.0) as u8;
            prev_scene.set_alpha_mod(alpha);
            canvas.copy(prev_scene, None, None).unwrap();
        } else {
            let alpha = (((progress - 0.5) * 2.0) * 255.0) as u8;
            scene.set_alpha_mod(alpha);
            canvas.copy(scene, None, None).unwrap();
        }
        None
    }

}

