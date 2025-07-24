use raytracer::*;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let e = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: (vector(1.0, 1.0, 0.0).normalize()) * 10.0,
    };

    let mut ticks = 0;

    while p.position.y > 0.0 {
        println!(
            "Time: {:?}, x: {:?}, y: {:?}",
            ticks, p.position.x, p.position.y
        );
        p.position = p.position + p.velocity;
        p.velocity = p.velocity + e.gravity + e.wind;
        ticks += 1;
    }
}
