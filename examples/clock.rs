use std::f64::consts::FRAC_PI_6;

use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::matrix::*;
use raytracer::point::*;

fn main() {
    plot_clock(1000, "./images/ppm/clock.ppm");
}

fn plot_clock(dim: usize, path: &str) {
    let mut canvas = Canvas::empty(dim as usize, dim as usize);
    let pixel_color = Color::new(1.0, 1.0, 1.0);

    let angle_inc = FRAC_PI_6;

    // radius of clock is 3/8 of a side dimension
    let r = 3.0 / 8.0 * dim as f64;

    // translation will be to the center
    let tc = Matrix::translation(dim as f64 / 2.0, dim as f64 / 2.0, 0.0);

    // translation to 12:00
    let tt = Matrix::translation(r, 0.0, 0.0);

    for i in 0..12 {
        // initialize the point to the origin
        // remember that it is not centered yet...
        // and the canvas is on x and y axis.  We are rotating around z
        let p = Point::new(0.0, 0.0, 0.0);

        // rotation will be some multiple of angle_inc
        let tr = Matrix::rotation_z(angle_inc * i as f64);

        // transformation order is:
        // 1.  move the point to the 12:00 position
        // 2.  rotate to the correct hour position (rotation is always around the origin)
        // 3.  translate the origin to the center of the canvas
        let h = tc * tr * tt * p;
        canvas.write_pixel(h.x as usize, h.y as usize, pixel_color);
    }

    canvas.write_ppm(path);
}
