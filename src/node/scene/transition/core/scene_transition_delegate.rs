use ::node::scene::transition::{ TransitionStatus };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };
use sdl2::render::{ Texture, BlendMode };

pub trait SceneTransitionDelegate {

    fn canvas_blend_mode(&self) -> BlendMode;

    fn render<'a>(&self, canvas: &mut Canvas<Window>, scene: &mut Texture<'a>, prev_scene: &mut Texture<'a>) -> TransitionStatus;

}
