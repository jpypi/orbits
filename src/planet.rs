use graphics::types::Color;
use graphics::draw_state::DrawState;

use graphics::math::*;
use graphics::*;

//const G: f64 = 6.67408e-11;           // m^3/(kg*s^2)
const G: f64 = 6.67408e-20;           // km^3/(kg*s^2)

pub struct Planet {
    pub color: Color,
    pub mass: f64,
    pub radius: f64,
    pub pos: Vec2d,
    pub vel: Vec2d,
    pub tiny: f64,
}
fn unit(vec: [f64;2]) -> [f64;2] {
    let len = dot(vec, vec).sqrt();
    mul_scalar(vec, 1.0 / len)
}

impl Planet {
    pub fn update(&self, universe: &Vec<Planet>, dt: f64) -> (Vec2d, Vec2d) {
        let mut net_force = [0.0, 0.0];

        //println!("My mass: {}", self.mass);
        for p in universe {
            if (p as *const _) != (self as *const _) {
                let dist = sub(self.pos, p.pos);
                let numer = self.mass * p.mass;
                let denom = dot(dist, dist);
                let force = mul_scalar(unit(dist), -G * numer / denom);
                net_force = add(net_force, force);
            }
        }
        //println!("net force: {:?}", net_force);

        let net_acc = mul_scalar(net_force, 1.0/self.mass);
        //println!("net acc: {:?}", net_acc);

        // Update position
        let pos = add(self.pos, mul_scalar(self.vel, dt));

        // Update velocity
        //let vel = self.vel;
        let vel = add(self.vel, mul_scalar(net_acc, dt));
        //println!("vel: {:?}", vel);

        return (pos, vel);
    }

    pub fn render<G: Graphics>(&self, draw_state: &DrawState,
                               pos_trans: Matrix2d,
                               zoomed_trans: Matrix2d,
                               gl: &mut G) {
        let e = Ellipse::new(self.color);
        //let e = Ellipse::new_border(self.color, self.radius/1.1);

        e.draw([self.pos[0] - self.radius, self.pos[1] - self.radius,
                self.radius * 2.0, self.radius * 2.0],
               draw_state, zoomed_trans, gl);

        if self.tiny > 0.0 {
            let r = self.tiny;
            let b = Ellipse::new_border(self.color, 0.5);
            let pos = mul_scalar(self.pos, 0.0018);
            b.draw([pos[0] - r, pos[1] - r, r * 2.0, r * 2.0],
                   draw_state, pos_trans, gl);
        }
    }
}
