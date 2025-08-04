use raytracer::camera::*;
use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::point::*;
use raytracer::ray::*;
use raytracer::sphere::*;
use raytracer::utils::create_progress_bar;

fn main() {
    render_scene(800, "./images/ppm/scene.ppm");
}

fn render_scene(dim: usize, path: &str) {
    let mut canvas = Canvas::empty(dim, dim);

    canvas.write_ppm(path);
}
