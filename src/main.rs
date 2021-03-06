#![allow(dead_code)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::{Window, WindowSettings, Size};
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::{GlyphCache};
use graphics::{Graphics};
use graphics::types::Color;
use graphics::character::CharacterCache;

mod planet;
use planet::Planet;

const BACKGROUND: Color = [0.2, 0.2, 0.2, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 0.8];

//http://curious.astro.cornell.edu/about-us/41-our-solar-system/the-earth/orbit/
//85-how-fast-does-the-earth-go-at-perihelion-and-aphelion-intermediate
const DIST_EART: f64 = 152.10e6; // km
const DIST_MOON: f64 = 384400.0; // km


pub struct App {
    gl: GlGraphics,
    universe: Vec<Planet>,
    zoom: f64,
}

impl App {
    fn render<C>(&mut self, args: &RenderArgs, size: Size, glyphs: &mut C)
        where C: CharacterCache<Texture=<GlGraphics as Graphics>::Texture> {

        use graphics::*;

        let universe = &self.universe;

        let zoom = self.zoom;
        self.gl.draw(args.viewport(), |ctr, gl| {
            clear(BACKGROUND, gl);

            let t = ctr.transform.trans((size.width as f64)/2.0,
                                        (size.height as f64)/2.0);
            let zt = t.scale(zoom, zoom);

            for p in universe {
                p.render(&ctr.draw_state, t, zt, zoom, gl);
            }

            let zoom_level_text = format!("Zoom: {}", zoom);
            let text_trans = ctr.transform.trans(2.0, size.height as f64 - 2.0);
            text(WHITE, 14, &zoom_level_text, &mut *glyphs, text_trans, gl);
            //normal_font.draw(&zoom_level_text, glyphs,
            //                 &ctr.draw_state, ctr.transform, gl);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut updates = Vec::new();

        for p in &self.universe {
            updates.push(p.update(&self.universe, args.dt * 100000.0));
        }

        for i in 0..self.universe.len() {
            self.universe[i].pos = updates[i].0;
            self.universe[i].vel = updates[i].1;
        }
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    // Create a Glutin window
    let mut window: GlutinWindow = WindowSettings::new("Orbits", [600, 1080]).samples(8)
                             .opengl(opengl).exit_on_esc(true).build().unwrap();

    let ref mut glyphs = GlyphCache::new("DejaVuSans.ttf")
                         .expect("Could not load font");


    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        universe: Vec::new(),
        zoom: 0.000004,
    };

    let sun = Planet {
        color: [0.8, 0.8, 0.0, 1.0],
        mass:  1.989e30,
        radius: 695_700.0,
        pos: [0.0, 0.0],
        vel: [0.0, 0.0],
        tiny: 0.0,
    };

    let earth = Planet {
        color: [0.1, 0.8, 0.15, 1.0],
        mass:  5.972e24,
        radius: 6_371.0,
        pos: [DIST_EART, 0.0],
        vel: [0.0, -29.3],
        tiny: 8.0,
    };

    let moon = Planet {
        color: [0.7, 0.7, 0.7, 1.0],
        mass:  7.34767309e22,
        radius: 1_737.0,
        pos: [DIST_EART + DIST_MOON, 0.0],
        vel: [0.0, earth.vel[1] + 1.0],
        tiny: 3.0,
    };

    app.universe.push(sun);
    app.universe.push(earth);
    app.universe.push(moon);


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, window.size(), glyphs);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        e.mouse_scroll(|_dx, dy| app.zoom += dy * 1e-6);
        /*
        if let Some(m) = e.mouse_scroll_args() {
            app.zoom += m[1] * 1e-6;
        }
        */
    }
}
