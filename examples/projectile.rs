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

fn log(ticks: u16, position: Point) {
    println!(
        "Time: {:?}, x: {:?}, y: {:?}",
        ticks, position.x, position.y
    );
}

fn tick(p: &mut Projectile, e: &Environment) {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + e.gravity + e.wind;
}

fn main() {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize();
    let mut p = Projectile::new(start, velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);

    let mut time = 0;

    while p.position.y > 0.0 {
        log(time, p.position);
        tick(&mut p, &e);
        time += 1;
    }
}
