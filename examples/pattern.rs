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
use std::f64::consts::*;

fn main() {
    render_scene(4000, 2000, "./images/ppm/pattern.ppm");
}

fn render_scene(hsize: usize, vsize: usize, path: &str) {
    // world
    let mut world = World::empty();

    // light
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
    world.light = light;

    // camera
    let mut camera = Camera::new(hsize, vsize, std::f64::consts::PI / 3.0);
    camera.transform = Matrix::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let stripe1 = Pattern::striped(Color::RED, Color::PINK).with_transform(
        Transformation::rotation_y(PI / 4.0) * Transformation::scaling(0.25, 0.25, 0.25),
    );

    let stripe2 = Pattern::striped(Color::DARK_GRAY, Color::GRAY).with_transform(
        Transformation::rotation_y(PI / -4.0) * Transformation::scaling(0.25, 0.25, 0.25),
    );

    let checker = Pattern::checker(stripe1, stripe2);
    let floor_material = Material {
        pattern: Some(checker),
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor = Shape::from(Plane::new()).with_material(floor_material.clone());
    world.objects.push(floor);

    // middle sphere
    let middle_sphere_material = Material {
        pattern: Some(Pattern::checker_uv(
            16.0,
            8.0,
            Color::GOLD,
            Color::STEEL_BLUE,
        )),
        color: Color::new(0.1, 1.0, 0.5),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let middle_sphere_transform = Transformation::translation(-0.5, 1.0, 0.5);
    let middle_sphere = Shape::from(Sphere::new())
        .with_material(middle_sphere_material)
        .with_transform(middle_sphere_transform);
    world.objects.push(middle_sphere);

    // right sphere
    let right_sphere_material = Material {
        color: Color::new(0.5, 1.0, 0.1),
        pattern: Some(
            Pattern::striped(Color::BLUE, Color::LIGHT_SEA_GREEN).with_transform(
                Transformation::rotation_z(-FRAC_PI_4) * Transformation::scaling(0.1, 0.1, 0.1),
            ),
        ),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let right_sphere_transform =
        Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5);
    let right_sphere = Shape::from(Sphere::new())
        .with_material(right_sphere_material)
        .with_transform(right_sphere_transform);
    world.objects.push(right_sphere);

    // left sphere
    let left_sphere_material = Material {
        color: Color::new(1.0, 0.8, 0.1),
        pattern: Some(
            Pattern::gradient(Color::PURPLE, Color::YELLOW).with_transform(
                Transformation::rotation_z(-FRAC_PI_4)
                    * Transformation::translation(-1.0, 0.0, 0.0)
                    * Transformation::scaling(2.0, 2.0, 2.0),
            ),
        ),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let left_sphere_transform =
        Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33);
    let left_sphere = Shape::from(Sphere::new())
        .with_material(left_sphere_material)
        .with_transform(left_sphere_transform);
    world.objects.push(left_sphere);

    let canvas = camera.render_with_progress(&world);
    canvas.write_ppm(path);
}
