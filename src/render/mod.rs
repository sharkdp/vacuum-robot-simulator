use opengl_graphics::GlGraphics;

use graphics::math::Matrix2d;
use graphics::line::Line;
use graphics::ellipse::Ellipse;
use graphics::{DrawState, Transformed};
use graphics::color;

use geometry;
use simulation::robot;
use pointcloud;

pub struct RenderConfig {
    pub scale: f64
}

impl RenderConfig {
    pub fn pixel_coords(&self, vec: geometry::Vector) -> (f64, f64) {
        (self.scale * (vec.x as f64), -self.scale * (vec.y as f64))
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub trait Draw {
    fn draw(&self,
            config: &RenderConfig,
            transform: Matrix2d,
            gl: &mut GlGraphics);
}

impl Draw for geometry::Line {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let line = Line::new(WHITE, 1.0);

        let (x1, y1) = config.pixel_coords(self.start);
        let (x2, y2) = config.pixel_coords(self.end);
        let coords: [f64; 4] = [x1, y1, x2, y2];

        line.draw(coords, &DrawState::default(), transform, gl);
    }
}

impl Draw for robot::Robot {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let robot_circ = Ellipse {
            color: color::hex("ffd42a"),
            border: None,
            resolution: 64
        };

        let robot_radius = config.scale;
        let (px, py) = config.pixel_coords(self.pose.position);

        robot_circ.draw([0.0, 0.0, robot_radius, robot_radius],
                        &Default::default(),
                        transform.trans(px - robot_radius / 2.0, py - robot_radius / 2.0),
                        gl)
    }
}

impl Draw for pointcloud::PointCloud {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let point = Ellipse {
            color: color::hex("1a1a1a"),
            border: None,
            resolution: 32
        };
        let point_radius = 0.25 * config.scale;
        for &p in self.iter() {
            let (px, py) = config.pixel_coords(p.pos);

            point.draw([0.0, 0.0, point_radius, point_radius],
                       &Default::default(),
                       transform.trans(px - point_radius / 2.0, py - point_radius / 2.0),
                       gl)
        }
    }
}
