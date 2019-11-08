use std::rc::Rc;
use ::node::scene::transition::{ SceneTransitionDelegate, TransitionStatus };
use ::util::{ context };
use ::util::parameter::{ Size };
use sdl2::render::{ Texture, BlendMode };
use sdl2::pixels::{ PixelFormatEnum, Color };

pub struct SceneTransition {
    delegate: Rc<dyn SceneTransitionDelegate>
}

impl SceneTransition {

    pub fn create<T>(callback: T) -> Rc<SceneTransition> where T: Fn() -> Rc<dyn SceneTransitionDelegate> {
        Rc::new(SceneTransition {
            delegate: callback()
        })
    }

    pub fn render<'a>(&self, mut scene: Texture<'a>, mut prev_scene: Texture<'a>) -> (TransitionStatus, Texture<'a>) {
        let query = scene.query();
        let mut canvas = context(|c| c.create_sub_canvas(Size::new(query.width, query.height)));
        canvas.set_blend_mode(self.delegate.canvas_blend_mode());
        let mut status = TransitionStatus::Wait;
        context(|c| &mut c.canvas).with_texture_canvas(&mut canvas, |c| {
            status = self.delegate.render(c, &mut scene, &mut prev_scene);
        });
        (status, canvas)
    }

}

