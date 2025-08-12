use raytracer::camera::*;
use raytracer::color::*;
use raytracer::material::Material;
use raytracer::matrix::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::shapes::{Shape, Sphere};
use raytracer::vector::*;
use raytracer::world::*;

fn main() {
    render_scene(1000, 500, "./images/ppm/scene.ppm");
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

    // floor
    let floor_material = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor_transform = Transformation::scaling(10.0, 0.01, 10.0);
    let floor = Shape::from(Sphere::new())
        .with_material(floor_material.clone())
        .with_transform(floor_transform);
    world.objects.push(floor);

    // left wall
    let left_wall_transform = Transformation::translation(0.0, 0.0, 5.0)
        * Transformation::rotation_y(std::f64::consts::PI / 4.0)
        * Transformation::rotation_z(std::f64::consts::PI / 2.0)
        * Transformation::scaling(10.0, 0.01, 10.0);

    let left_wall = Shape::from(Sphere::new())
        .with_material(floor_material.clone())
        .with_transform(left_wall_transform);
    world.objects.push(left_wall);

    // right wall
    let right_wall_transform = Transformation::translation(0.0, 0.0, 5.0)
        * Transformation::rotation_y(std::f64::consts::PI / 4.0)
        * Transformation::rotation_x(std::f64::consts::PI / 2.0)
        * Transformation::scaling(10.0, 0.01, 10.0);

    let right_wall = Shape::from(Sphere::new())
        .with_material(floor_material)
        .with_transform(right_wall_transform);
    world.objects.push(right_wall);

    // middle sphere
    let middle_sphere_material = Material {
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
