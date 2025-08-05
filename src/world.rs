// TODO - Mutating objects in the world is clunky - revisit and make more rustic

use super::color::Color;
use super::intersection::Computations;
use super::intersection::Intersections;
use super::material::Material;
use super::matrix::Transformation;
use super::point::Point;
use super::point_light::PointLight;
use super::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> Self {
        Self { objects, light }
    }

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            light: PointLight::new(Point::ORIGIN, Color::BLACK),
        }
    }

    pub fn intersections<'a>(&'a self, ray: Ray) -> Intersections<'a> {
        let mut intersections = Intersections::empty();
        for object in &self.objects {
            intersections.extend(object.intersect(ray));
        }
        intersections
    }

    fn shade_hit(&self, comps: Computations) -> Color {
        comps.object.lighting(
            comps.point,
            self.light,
            comps.eye_vector,
            comps.normal_vector,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersections(ray);
        let hit = intersections.hit();

        match hit {
            Some(hit) => {
                let comps = hit.prepare_computations(ray);
                self.shade_hit(comps)
            }
            None => Color::BLACK,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
        let material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let sphere1 = Sphere::new_with_identity_transform(material);
        let sphere2 = Sphere::new_with_default_material(Transformation::scaling(0.5, 0.5, 0.5));
        let objects = vec![sphere1, sphere2];
        Self::new(objects, light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_world_creation() {
        let world = World::empty();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.light.intensity, Color::BLACK);
    }

    #[test]
    fn test_default_world() {
        let world = World::default();

        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
        let material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let sphere1 = Sphere::new_with_identity_transform(material);
        let sphere2 = Sphere::new_with_default_material(Transformation::scaling(0.5, 0.5, 0.5));

        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.light, light);
        assert_eq!(world.objects[0], sphere1);
        assert_eq!(world.objects[1], sphere2);
    }

    #[test]
    fn intersect_world_with_ray() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = world.intersections(ray);

        assert_eq!(intersections.all().len(), 4);
        assert_eq!(intersections.all()[0].t, 4.0);
        assert_eq!(intersections.all()[1].t, 4.5);
        assert_eq!(intersections.all()[2].t, 5.5);
        assert_eq!(intersections.all()[3].t, 6.0);
    }

    #[test]
    fn color_at_no_intersections() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let color = world.color_at(ray);

        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn color_at_with_intersections() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let color = world.color_at(ray);

        assert_abs_diff_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_when_intersection_behind_ray() {
        let mut world = World::default();
        world.objects[0].material_mut().ambient = 1.0;
        world.objects[1].material_mut().ambient = 1.0;

        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let color = world.color_at(ray);

        assert_abs_diff_eq!(color, world.objects[1].material().color);
    }
}
