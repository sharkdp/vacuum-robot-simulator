extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::Transformed;

pub mod controller;
pub mod geometry;
pub mod math;
pub mod pointcloud;
pub mod render;
pub mod simulation;

use math::Scalar;
use geometry::{Vector, Line, Pose};
use pointcloud::PointCloud;
use controller::Controller;
use simulation::robot::Robot;
use simulation::sensor::laserscanner::LaserScanner;

use render::{RenderConfig, Draw};

struct App {
    gl: GlGraphics,
    render_config: RenderConfig,
    robot: Robot,
    pointcloud: PointCloud,
    controller: Controller,
    objects: Vec<Line>
}

const COLOR_BG: [f32; 4] = [0.17, 0.35, 0.62, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let (x, y) = (f64::from(args.width / 2),
                      f64::from(args.height / 2));

        // Clear screen
        graphics::clear(COLOR_BG, &mut self.gl);

        let render_config = &self.render_config;

        let objects = &self.objects;
        let robot = &self.robot;
        let pointcloud = &self.pointcloud;
        let controller = &self.controller;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            // Draw all static objects
            for o in objects {
                o.draw(render_config, transform, gl);
            }

            // Draw robot
            robot.draw(render_config, transform, gl);

            // Draw current LiDAR measurements
            pointcloud.draw(render_config, transform, gl);

            // Draw the internal state of the controller
            controller.draw(render_config, transform, gl);
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        // Perform a laser scan
        self.pointcloud = self.robot.laser_scanner.scan(&self.robot.pose, &self.objects);

        // Run the perception algorithm
        self.controller.cycle(&self.pointcloud);

        // Move the robot (TODO)
        self.robot.pose.position.y += 0.001;
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

    // TODO: check this
    window.set_ups(60);
    window.set_max_fps(60);

    // Little helper to construct vectors
    let vec = |x, y| Vector::new(Scalar::from(x), Scalar::from(y));

    let mut app = App {
        gl: GlGraphics::new(opengl),
        render_config: RenderConfig {
            scale: 20.0
        },
        pointcloud: PointCloud::empty(),
        robot: Robot {
            pose: Pose::new(vec(1, 1), 0.0),
            laser_scanner: LaserScanner {
                num_columns: 100
            }
        },
        controller: Controller::default(),
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

    app.objects.push(Line::new(vec(4, 4), vec(5, 4)));
    app.objects.push(Line::new(vec(5, 4), vec(5, 5)));
    app.objects.push(Line::new(vec(5, 5), vec(4, 5)));
    app.objects.push(Line::new(vec(4, 5), vec(4, 4)));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(a) = e.render_args() {
            app.render(&a);
        }

        if let Some(a) = e.update_args() {
            app.update(&a);
        }

        if let Some(a) = e.mouse_scroll_args() {
            app.render_config.scale *= 1.0 + 0.2 * a[1];
            app.render_config.scale = f64::max(1.0, app.render_config.scale);
        }
    }
}
