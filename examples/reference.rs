use raytracer::camera::*;
use raytracer::color::*;
use raytracer::material::Material;
use raytracer::matrix::*;
use raytracer::pattern::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::shapes::{Shape, Sphere};
use raytracer::vector::*;
use raytracer::world::*;

fn main() {
    render_scene(800, 800, "./images/ppm/reference.ppm");
}

fn render_scene(hsize: usize, vsize: usize, path: &str) {
    // world
    let mut world = World::empty();

    // light
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
    world.light = light;

    // camera
    let mut camera = Camera::new(hsize, vsize, 1.0);
    camera.transform = Matrix::view_transform(
        Point::new(0.0, 0.0, -3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let sphere_material = Material {
        color: Color::new(0.1, 1.0, 0.5),
        pattern: Some(Pattern::checker_uv(
            20.0,
            10.0,
            Color::new(0.0, 0.5, 0.0),
            Color::WHITE,
        )),
        diffuse: 0.6,
        specular: 0.4,
        ambient: 0.1,
        shininess: 10.0,
    };

    let sphere = Shape::from(Sphere::new()).with_material(sphere_material);
    world.objects.push(sphere);

    let canvas = camera.render_with_progress(&world);
    canvas.write_ppm(path);
}
