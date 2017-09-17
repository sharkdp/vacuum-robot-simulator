extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::Transformed;

pub mod geometry;
pub mod pointcloud;
pub mod renderer;
pub mod simulation;
pub mod math;

use math::Scalar;
use geometry::{Vector, Line, Pose};
use simulation::robot::Robot;
use simulation::sensor::laserscanner::LaserScanner;

use renderer::{RendererConfig, Draw};

struct App {
    gl: GlGraphics,
    config: RendererConfig,
    robot: Robot,
    objects: Vec<Line>
}

const COLOR_BG: [f32; 4] = [0.17, 0.35, 0.62, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        // Clear screen
        graphics::clear(COLOR_BG, &mut self.gl);

        let config = &self.config;

        // Draw all static objects
        for ref o in &self.objects {
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(x, y);
                o.draw(config, transform, gl);
            });
        }

        // Draw robot
        use graphics::ellipse::Ellipse;
        let robot_circ = Ellipse {
            color: graphics::color::hex("ffd42a"),
            border: None,
            resolution: 16
        };

        let robot_radius = config.scale;
        let (px, py) = self.config.pixel_coords(self.robot.pose.position);
        self.gl.draw(args.viewport(), |c, gl| {
            robot_circ.draw([0.0, 0.0, robot_radius, robot_radius],
                            &Default::default(),
                            c.trans(x + px - robot_radius / 2.0, y + py - robot_radius / 2.0).transform,
                            gl)
        });

        // TODO: this is just for testing
        let pointcloud = self.robot.laser_scanner.scan(&self.robot.pose, &self.objects);

        let point = Ellipse {
            color: graphics::color::hex("1a1a1a"),
            border: None,
            resolution: 32
        };
        let point_radius = 0.25 * config.scale;
        for &p in pointcloud.iter() {
            let (px, py) = self.config.pixel_coords(p.pos);
            self.gl.draw(args.viewport(), |c, gl| {
                point.draw([0.0, 0.0, point_radius, point_radius],
                           &Default::default(),
                           c.trans(x + px - point_radius / 2.0, y + py - point_radius / 2.0).transform,
                           gl)
            });
        }
    }

    fn update(&mut self, _: &UpdateArgs) {
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Vacuum Robot Simulator",
            [800, 400]
        )
        .opengl(opengl)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Little helper to construct vectors
    let vec = |x, y| Vector::new(x as Scalar, y as Scalar);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        config: RendererConfig {
            scale: 30.0
        },
        robot: Robot {
            pose: Pose::new(vec(1, 1), 0.0),
            laser_scanner: LaserScanner {
                num_columns: 100
            }
        },
        objects: vec!()
    };

    app.objects.push(Line::new(vec(0, 0), vec(10, 0)));
    app.objects.push(Line::new(vec(10, 0), vec(10, 10)));
    app.objects.push(Line::new(vec(10, 10), vec(0, 10)));
    app.objects.push(Line::new(vec(0, 10), vec(0, 2)));
    app.objects.push(Line::new(vec(0, 2), vec(-10, 2)));
    app.objects.push(Line::new(vec(-10, 2), vec(-10, -12)));
    app.objects.push(Line::new(vec(-10, -12), vec(0, -12)));
    app.objects.push(Line::new(vec(0, -12), vec(0, 0)));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
