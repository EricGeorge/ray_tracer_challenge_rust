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
    render_scene(4000, 2000, "./images/ppm/reflection_refraction.ppm");
}

fn render_scene(hsize: usize, vsize: usize, path: &str) {
    // world
    let mut world = World::empty();

    // camera
    let mut camera = Camera::new(hsize, vsize, 1.152);
    camera.transform = Matrix::view_transform(
        Point::new(-2.6, 1.5, -3.9),
        Point::new(-0.6, 1.0, -0.8),
        Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light = PointLight::new(Point::new(-4.9, 4.9, -1.0), Color::WHITE);
    world.light = light;

    // wall material
    let wall_material = Material {
        color: Color::WHITE,
        pattern: Some(
            Pattern::striped(Color::new(0.45, 0.45, 0.45), Color::new(0.55, 0.55, 0.55))
                .with_transform(
                    Transformation::scaling(0.25, 0.25, 0.25)
                        * Transformation::rotation_y(FRAC_PI_2),
                ),
        ),
        ambient: 0.0,
        diffuse: 0.4,
        specular: 0.0,
        reflective: 0.3,
        ..Default::default()
    };

    // floor
    let floor = Shape::from(Plane::new())
        .with_transform(Transformation::rotation_y(0.31415))
        .with_material(Material {
            color: Color::WHITE,
            pattern: Some(Pattern::checker(
                Color::new(0.35, 0.35, 0.35),
                Color::new(0.65, 0.65, 0.65),
            )),
            specular: 0.0,
            reflective: 0.4,
            ..Default::default()
        });
    world.objects.push(floor);

    // ceiling
    let ceiling = Shape::from(Plane::new())
        .with_transform(Transformation::translation(0.0, 5.0, 0.0))
        .with_material(Material {
            color: Color::new(0.8, 0.8, 0.8),
            ambient: 0.3,
            specular: 0.0,
            ..Default::default()
        });
    world.objects.push(ceiling);

    // west wall
    let west_wall = Shape::from(Plane::new())
        .with_transform(Transformation::translation(-5.0, 0.0, 0.0))
        .with_material(wall_material.clone());
    world.objects.push(west_wall);

    // east wall
    let east_wall = Shape::from(Plane::new())
        .with_transform(
            Transformation::translation(5.0, 0.0, 0.0)
                * Transformation::rotation_z(FRAC_PI_2)
                * Transformation::rotation_y(FRAC_PI_2),
        )
        .with_material(wall_material.clone());
    world.objects.push(east_wall);

    // north wall
    let north_wall = Shape::from(Plane::new())
        .with_transform(
            Transformation::translation(0.0, 0.0, 5.0) * Transformation::rotation_x(FRAC_PI_2),
        )
        .with_material(wall_material.clone());
    world.objects.push(north_wall);

    // south wall
    let south_wall = Shape::from(Plane::new())
        .with_transform(
            Transformation::translation(0.0, 0.0, -5.0) * Transformation::rotation_x(FRAC_PI_2),
        )
        .with_material(wall_material.clone());
    world.objects.push(south_wall);

    // BACKGROUND BALLS
    // green sphere
    let green_sphere = Shape::from(Sphere::new())
        .with_transform(
            Transformation::translation(-1.0, 0.5, 4.5) * Transformation::scaling(0.5, 0.5, 0.5),
        )
        .with_material(Material {
            color: Color::new(0.4, 0.9, 0.6),
            shininess: 50.0,
            ..Default::default()
        });
    world.objects.push(green_sphere);

    // blue sphere
    let blue_sphere = Shape::from(Sphere::new())
        .with_transform(
            Transformation::translation(-1.7, 0.3, 4.7) * Transformation::scaling(0.3, 0.3, 0.3),
        )
        .with_material(Material {
            color: Color::new(0.4, 0.6, 0.9),
            shininess: 50.0,
            ..Default::default()
        });
    world.objects.push(blue_sphere);

    // brown sphere
    let brown_sphere = Shape::from(Sphere::new())
        .with_transform(
            Transformation::translation(4.6, 0.4, 1.0) * Transformation::scaling(0.4, 0.4, 0.4),
        )
        .with_material(Material {
            color: Color::new(0.8, 0.5, 0.3),
            shininess: 50.0,
            ..Default::default()
        });
    world.objects.push(brown_sphere);

    // pink sphere
    let pink_sphere = Shape::from(Sphere::new())
        .with_transform(
            Transformation::translation(4.7, 0.3, 0.4) * Transformation::scaling(0.3, 0.3, 0.3),
        )
        .with_material(Material {
            color: Color::new(0.9, 0.4, 0.5),
            shininess: 50.0,
            ..Default::default()
        });
    world.objects.push(pink_sphere);

    // FOREGROUND BALLS
    // red sphere
    let red_sphere = Shape::from(Sphere::new())
        .with_transform(Transformation::translation(-0.6, 1.0, 0.6))
        .with_material(Material {
            color: Color::new(1.0, 0.3, 0.2),
            specular: 0.4,
            shininess: 5.0,
            ..Default::default()
        });
    world.objects.push(red_sphere);

    //     # blue glass sphere
    // - add: sphere
    //   transform:
    //     - [ scale, 0.7, 0.7, 0.7 ]
    //     - [ translate, 0.6, 0.7, -0.6 ]
    //   material:
    //     color: [0, 0, 0.2]
    //     ambient: 0
    //     diffuse: 0.4
    //     specular: 0.9
    //     shininess: 300
    //     reflective: 0.9
    //     transparency: 0.9
    //     refractive-index: 1.5

    // # green glass sphere
    // - add: sphere
    //   transform:
    //     - [ scale, 0.5, 0.5, 0.5 ]
    //     - [ translate, -0.7, 0.5, -0.8 ]
    //   material:
    //     color: [0, 0.2, 0]
    //     ambient: 0
    //     diffuse: 0.4
    //     specular: 0.9
    //     shininess: 300
    //     reflective: 0.9
    //     transparency: 0.9
    //     refractive-index: 1.5

    let canvas = camera.render_with_progress(&world);
    canvas.write_ppm(path);
}
