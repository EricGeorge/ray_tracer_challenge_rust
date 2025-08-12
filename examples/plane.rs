use raytracer::camera::*;
use raytracer::color::*;
use raytracer::material::Material;
use raytracer::matrix::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::shapes::Plane;
use raytracer::shapes::{Shape, Sphere};
use raytracer::vector::*;
use raytracer::world::*;

fn main() {
    render_scene(4000, 2000, "./images/ppm/plane.ppm");
}

fn render_scene(hsize: usize, vsize: usize, path: &str) {
    // progress bar setup
    // let pb = create_progress_bar((hsize * vsize) as u64);

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

    let floor = Shape::from(Plane::new()).with_material(floor_material.clone());
    world.objects.push(floor);

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

    // let canvas = camera.render(&world, || {
    //     pb.inc(1);
    // });

    let canvas = camera.render_with_progress(&world);
    canvas.write_ppm(path);
}
