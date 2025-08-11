use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::point::*;
use raytracer::ray::*;
use raytracer::shapes::{Shape, Sphere};
use raytracer::utils::create_progress_bar;

fn main() {
    plot_circle(800, "./images/ppm/circle.ppm");
}

fn plot_circle(dim: usize, path: &str) {
    let canvas_pixels = dim;
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let red = Color::new(1.0, 0.0, 0.0);

    let mut canvas = Canvas::empty(canvas_pixels, canvas_pixels);

    let s = Shape::from(Sphere::default());

    // progress bar setup
    let pb = create_progress_bar((canvas_pixels * canvas_pixels) as u64);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let i = s.intersect(r);

            if i.hit().is_some() {
                canvas.write_pixel(x, y, red);
            }

            // update progress bar
            pb.inc(1);
        }
    }
    canvas.write_ppm(path);
}
