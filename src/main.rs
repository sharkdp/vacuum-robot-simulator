extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate svg2polylines;

use std::env;
use std::fs;
use std::io::Read;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::Transformed;
use svg2polylines::Polyline;

pub mod controller;
pub mod geometry;
pub mod math;
pub mod pointcloud;
pub mod render;
pub mod sensor;
pub mod simulation;

use controller::Controller;
use geometry::{Vector, Line, Pose};
use math::Scalar;
use sensor::laserscanner::Scan;
use simulation::robot::Robot;
use simulation::sensor::laserscanner::LaserScanner;

use render::{RenderConfig, Draw};

struct App {
    gl: GlGraphics,
    render_config: RenderConfig,
    robot: Robot,
    last_scan: Scan,
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
        let pointcloud = self.last_scan.to_pointcloud(&robot.pose);
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
        self.last_scan = self.robot.laser_scanner.scan(&self.robot.pose, &self.objects);

        // Run the perception algorithm
        self.controller.cycle(&self.last_scan, &self.robot.pose);

        // Move the robot (TODO)
        self.robot.pose.position.y += 0.003;
        // self.robot.pose.heading -= 0.0001;
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
        last_scan: Scan::empty(),
        robot: Robot {
            pose: Pose::new(vec(1, 1), 0.2),
            laser_scanner: LaserScanner {
                num_columns: 100
            }
        },
        controller: Controller::default(),
        objects: vec!()
    };

    // Read static world from SVG file
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {},
        _ => {
            println!("Usage: {} <map.svg>", args[0]);
            std::process::exit(1);
        },
    };

    let mut file = fs::File::open(&args[1]).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Parse data
    let polylines: Vec<Polyline> = svg2polylines::parse(&s).unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });

    let m_per_px = 0.02;
    for polyline in &polylines {
        for pair in polyline.windows(2) {
            app.objects.push(
                Line::new(
                    Vector::new(pair[0].x * m_per_px, -pair[0].y * m_per_px),
                    Vector::new(pair[1].x * m_per_px, -pair[1].y * m_per_px)
                )
            )
        }
    }

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
