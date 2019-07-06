use sdl2::surface::{ Surface, SurfaceContext };
use sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;

pub struct VirtualCanvas {
    point: Point,
    inner_canvas: Canvas<Surface<'static>>,
    texture_creator: TextureCreator<SurfaceContext<'static>>,
}

impl VirtualCanvas {

    pub fn new(point: Point, width: u32, height: u32) -> Result<Self, String> {
        let surface = Surface::new(width, height, PixelFormatEnum::RGBA8888)?;
        let mut canvas = surface.into_canvas()?;
        canvas.set_blend_mode(BlendMode::Blend);
        Ok(Self {
            point: point,
            texture_creator: canvas.texture_creator(),
            inner_canvas: canvas,
        })
    }

    pub fn render(&mut self) -> Result<Texture, String> {
        self.inner_canvas.clear();
        let texture = self.texture_creator.create_texture_from_surface(self.inner_canvas.surface());
        texture.map_err(|e| e.to_string())
    }

}

