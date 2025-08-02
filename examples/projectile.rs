use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::point::*;
use raytracer::vector::*;

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

fn main() {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 16.0;
    let p = Projectile::new(start, velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);

    plot_trajectory(p, &e, "./images/ppm/trajectory.ppm");
}

fn plot_trajectory(mut p: Projectile, e: &Environment, path: &str) {
    let mut canvas = Canvas::empty(1800, 1100);
    let pixel_color = Color::new(0.85, 0.35, 0.40);

    while p.position.y > 0.0 {
        canvas.write_pixel(
            p.position.x.round() as usize,
            (1100_f64 - p.position.y).round() as usize,
            pixel_color,
        );
        p = tick(p, e);
    }

    canvas.write_ppm(path);
}

fn tick(proj: Projectile, env: &Environment) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
