use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::material::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::ray::*;
use raytracer::sphere::*;
use raytracer::utils::create_progress_bar;

fn main() {
    render_shaded_sphere(800, "./images/ppm/shading.ppm");
}

fn render_shaded_sphere(dim: usize, path: &str) {
    let canvas_pixels = dim;
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let mut canvas = Canvas::empty(canvas_pixels, canvas_pixels);

    // first setup the material
    let material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ..Default::default()
    };
    let s = Sphere::new().with_material(material);

    // then add the light source
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::WHITE;
    let light = PointLight::new(light_position, light_color);

    // progress bar setup
    let pb = create_progress_bar((canvas_pixels * canvas_pixels) as u64);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let i = s.intersect(r);

            if let Some(hit) = i.hit() {
                let point = r.position(hit.t);
                let normal = s.normal_at(point);
                let eye = -r.direction;

                let color = s.lighting(point, light, eye, normal, false);
                canvas.write_pixel(x, y, color);
            }

            // update progress bar
            pb.inc(1);
        }
    }
    canvas.write_ppm(path);
}
