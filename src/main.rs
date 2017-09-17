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

use geometry::vector::Vector;
use geometry::line::Line;

use renderer::{RendererConfig, Draw};

struct App {
    gl: GlGraphics,
    config: RendererConfig,
    objects: Vec<Box<Draw>>
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
    }

    fn update(&mut self, args: &UpdateArgs) {
        println!("FPS: {}", 1.0 / args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Vacuum Robot Simulator",
            [800, 400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        config: RendererConfig {
            scale: 10.0
        },
        objects: vec!()
    };

    // Little helper to construct vectors
    let vec = |x, y| Vector::new(x as f64, y as f64);

    let line1 = Line::new(vec(0, 0), vec(10, 0));
    let line2 = Line::new(vec(10, 0), vec(10, 10));
    let line3 = Line::new(vec(10, 10), vec(0, 10));
    let line4 = Line::new(vec(0, 10), vec(0, 2));
    let line5 = Line::new(vec(0, 2), vec(-10, 2));
    let line6 = Line::new(vec(-10, 2), vec(-10, -12));
    let line7 = Line::new(vec(-10, -12), vec(0, -12));
    let line8 = Line::new(vec(0, -12), vec(0, 0));

    app.objects.push(Box::new(line1));
    app.objects.push(Box::new(line2));
    app.objects.push(Box::new(line3));
    app.objects.push(Box::new(line4));
    app.objects.push(Box::new(line5));
    app.objects.push(Box::new(line6));
    app.objects.push(Box::new(line7));
    app.objects.push(Box::new(line8));

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
