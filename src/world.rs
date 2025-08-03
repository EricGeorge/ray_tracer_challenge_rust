use super::color::Color;
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
    pub light: Option<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: Option<PointLight>) -> Self {
        Self { objects, light }
    }

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn intersections(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections::empty();
        for object in &self.objects {
            intersections.extend(object.intersect(ray));
        }
        intersections
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
        Self::new(objects, Some(light))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn test_world_creation() {
        let world = World::empty();
        assert_eq!(world.objects.len(), 0);
        assert!(world.light.is_none());
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
        assert!(world.light.is_some());
        assert_eq!(world.light, Some(light));
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
}
