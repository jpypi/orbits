#![allow(dead_code)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::types::Color;

mod planet;
use planet::Planet;

const BACKGROUND: Color = [0.2, 0.2, 0.2, 1.0];

// G * (m1*m2)/r^2
const G: f64 = 6.67408e-11;           // m^3/(kg*s^2)
const MASS_SUN : f64 = 1.989e30;      // kg
const MASS_EART: f64 = 5.972e24;      // kg
//const MASS_MOON: f64 = 7.34767309e22; // kg
const MASS_MOON: f64 = 7.34767309e20; // kg

const DIST_EART: f64 = 146e6;         // km
const DIST_MOON: f64 = 384400.0;      // km


pub struct App {
    gl: GlGraphics,
    universe: Vec<Planet>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let universe = &self.universe;
        self.gl.draw(args.viewport(), |ctr, gl| {
            clear(BACKGROUND, gl);

            let zoom = 0.0015;
            let t = ctr.transform.trans(2560.0/2.0, 1080.0/2.0).scale(zoom, zoom);

            for p in universe {
                p.render(&ctr.draw_state, t, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut updates = Vec::new();

        for p in &self.universe {
            updates.push(p.update(&self.universe, args.dt * 100.0));
        }

        for i in 0..self.universe.len() {
            self.universe[i].pos = updates[i].0;
            self.universe[i].vel = updates[i].1;
            println!("new p: {:?}", self.universe[i].pos);
        }
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    // Create a Glutin window
    let mut window: Window = WindowSettings::new("Orbits", [600, 1080]).samples(8)
                             .opengl(opengl).exit_on_esc(true).build().unwrap();

    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        universe: Vec::new(),
    };

    let sun = Planet {
        color: [0.8, 0.8, 0.0, 1.0],
        mass:  MASS_SUN,
        radius: 695_700.0,
        pos: [0.0, 0.0],
        vel: [0.0, 0.0],
        tiny: 0.0,
    };

    let BASE = 0.0;

    let earth = Planet {
        color: [0.1, 0.8, 0.15, 1.0],
        mass:  MASS_EART,
        radius: 6_371.0,
        pos: [BASE, 0.0],
        //pos: [DIST_EART/12.0, 0.0],
        //pos: [DIST_EART/6.0, 0.0],
        vel: [0.0, 0.0],
        tiny: 0.0,//100.0,
    };

    let moon = Planet {
        color: [0.7, 0.7, 0.7, 1.0],
        mass:  MASS_MOON,
        radius: 1_737.0,
        pos: [BASE + DIST_MOON, 0.0],
        vel: [0.0, -8.0],
        tiny: 0.0,
    };

    //app.universe.push(sun);
    app.universe.push(earth);
    app.universe.push(moon);


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
