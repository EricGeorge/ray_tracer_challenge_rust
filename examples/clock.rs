use std::f64::consts::FRAC_PI_6;

use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::matrix::*;
use raytracer::point::*;
// use raytracer::vector::*;

fn main() {
    let dim = 400.0;
    let mut canvas = Canvas::empty(dim as usize, dim as usize);
    let pixel_color = Color::new(1.0, 1.0, 1.0);

    let angle_inc = FRAC_PI_6;

    // radius is 3/8 of a side dimension
    let r = 3.0 / 8.0 * dim;

    for i in 0..12 {
        // initialize the point to the 12:00 position
        // remember that it is not centered yet...
        // and the canvas is on x and y axis.  We are rotating around z
        let p = Point::new(r, 0.0, 0.0);

        // translation will be to the center
        let tt = Matrix::translation(dim / 2.0, dim / 2.0, 0.0);

        // rotation will be some multiple of angle_inc
        let tr = Matrix::rotation_z(angle_inc * i as f64);

        // order doesn't seem to matter here...
        let t = tr * tt;

        let h = t * p;
        canvas.write_pixel(h.x.round() as usize, h.y.round() as usize, pixel_color);
    }

    canvas.write_ppm("./clock.ppm");
}
