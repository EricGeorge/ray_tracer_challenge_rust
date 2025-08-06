// TODO - Mutating objects in the world is clunky - revisit and make more rustic

use super::color::Color;
use super::intersection::{Computations, Intersections};
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
        let shadowed = self.is_shadowed(comps.over_point);
        comps.object.lighting(
            comps.point,
            self.light,
            comps.eye_vector,
            comps.normal_vector,
            shadowed,
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

    pub fn is_shadowed(&self, point: Point) -> bool {
        let vector_to_light = self.light.position - point;
        let distance_to_light = vector_to_light.magnitude();
        let direction_to_light = vector_to_light.normalize();

        let shadow_ray = Ray::new(point, direction_to_light);
        let intersections = self.intersections(shadow_ray);

        if let Some(hit) = intersections.hit() {
            hit.t < distance_to_light
        } else {
            false
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
        let sphere1 = Sphere::new().with_material(material);
        let sphere2 = Sphere::new().with_transform(Transformation::scaling(0.5, 0.5, 0.5));
        let objects = vec![sphere1, sphere2];
        Self::new(objects, light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersection;
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
        let sphere1 = Sphere::new().with_material(material);
        let sphere2 = Sphere::new().with_transform(Transformation::scaling(0.5, 0.5, 0.5));

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

    #[test]
    fn no_shadow_when_nothing_is_collinear() {
        let world = World::default();
        let point = Point::new(0.0, 10.0, 0.0);
        let in_shadow = world.is_shadowed(point);

        assert!(!in_shadow);
    }

    #[test]
    fn shadowed_when_object_between_light_and_point() {
        let world = World::default();
        let point = Point::new(10.0, -10.0, 10.0);
        let in_shadow = world.is_shadowed(point);

        assert!(in_shadow);
    }

    #[test]
    fn not_shadowed_when_object_behind_light() {
        let world = World::default();
        let point = Point::new(-20.0, 20.0, -20.0);
        let in_shadow = world.is_shadowed(point);

        assert!(!in_shadow);
    }

    #[test]
    fn not_shadowed_when_object_behind_point() {
        let world = World::default();
        let point = Point::new(-2.0, 2.0, -2.0);
        let in_shadow = world.is_shadowed(point);

        assert!(!in_shadow);
    }

    #[test]
    fn shade_hit_with_shadow() {
        let mut world = World::default();
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        world.light = light;

        let s1 = Sphere::new();
        world.objects.push(s1.clone());

        let s2 = Sphere::new().with_transform(Transformation::translation(0.0, 0.0, 10.0));
        world.objects.push(s2);

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &s1);
        let comps = i.prepare_computations(r);
        let c = world.shade_hit(comps);

        assert_abs_diff_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().with_transform(Transformation::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &s);
        let comps = i.prepare_computations(r);

        assert_abs_diff_eq!(comps.over_point.z, -1e-5);
    }
}
