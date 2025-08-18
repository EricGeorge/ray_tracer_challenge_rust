use std::f64::consts::FRAC_PI_2;

use raytracer::camera::*;
use raytracer::color::*;
use raytracer::material::Material;
use raytracer::matrix::*;
use raytracer::pattern::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::shapes::Plane;
use raytracer::shapes::{Shape, Sphere};
use raytracer::vector::*;
use raytracer::world::*;

fn main() {
    render_scene(2000, 2000, "./images/ppm/fresnel.ppm");
}

fn render_scene(hsize: usize, vsize: usize, path: &str) {
    // world
    let mut world = World::empty();

    let mut camera = Camera::new(hsize, vsize, 0.45);
    camera.transform = Matrix::view_transform(
        Point::new(0.0, 0.0, -5.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let light = PointLight::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));
    world.light = light;

    // wall
    let wall_material = Material {
        pattern: Some(Pattern::checker(
            Color::new(0.15, 0.15, 0.15),
            Color::new(0.85, 0.85, 0.85),
        )),
        ambient: 0.8,
        diffuse: 0.2,
        specular: 0.0,
        ..Default::default()
    };

    let wall = Shape::from(Plane::new())
        .with_transform(
            Transformation::translation(0.0, 0.0, 10.0) * Transformation::rotation_x(FRAC_PI_2),
        )
        .with_material(wall_material);
    world.objects.push(wall);

    // glass sphere
    let glass_sphere_material = Material {
        color: Color::WHITE,
        pattern: None,
        ambient: 0.0,
        diffuse: 0.0,
        specular: 0.9,
        shininess: 300.0,
        reflective: 0.9,
        // transparency: 0.9,
        // refractive_index: 1.5,
    };

    let glass_sphere = Shape::from(Sphere::new()).with_material(glass_sphere_material);
    world.objects.push(glass_sphere);

    // hollow center
    let hollow_center_material = Material {
        color: Color::WHITE,
        pattern: None,
        ambient: 0.0,
        diffuse: 0.0,
        specular: 0.9,
        shininess: 300.0,
        reflective: 0.9,
        // transparency: 0.9,
        // refractive_index: 1.0000034,
    };

    let hollow_center = Shape::from(Sphere::new())
        .with_transform(Transformation::scaling(0.5, 0.5, 0.5))
        .with_material(hollow_center_material);
    world.objects.push(hollow_center);

    let canvas = camera.render_with_progress(&world);
    canvas.write_ppm(path);
}
