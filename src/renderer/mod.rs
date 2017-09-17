extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;

use graphics::math::Matrix2d;
use graphics::line::Line;
use graphics::DrawState;

use geometry;

pub struct RendererConfig {
    pub scale: f64
}

impl RendererConfig {
    pub fn pixel_coords(&self, vec: geometry::vector::Vector) -> (f64, f64) {
        (self.scale * (vec.x as f64), -self.scale * (vec.y as f64))
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub trait Draw {
    fn draw(&self,
            config: &RendererConfig,
            transform: Matrix2d,
            gl: &mut GlGraphics);
}

impl Draw for geometry::line::Line {
    fn draw(&self, config: &RendererConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let line = Line::new(WHITE, 1.0);

        let (x1, y1) = config.pixel_coords(self.start);
        let (x2, y2) = config.pixel_coords(self.end);
        let coords: [f64; 4] = [x1, y1, x2, y2];

        line.draw(coords, &DrawState::new_alpha(), transform, gl);
    }
}
