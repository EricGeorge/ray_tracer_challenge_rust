use crate::color::Color;
use crate::intersection::{Computations, Intersections};
use crate::point::Point;
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::shapes::Shape;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Shape>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Shape>, light: PointLight) -> Self {
        Self { objects, light }
    }

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            light: PointLight::new(Point::ORIGIN, Color::BLACK),
        }
    }

    // pub fn intersections<'a>(&'a self, ray: Ray) -> Intersections<'a> {
    //     let mut intersections = Intersections::empty();
    //     for object in &self.objects {
    //         intersections.extend(object.intersect(ray));
    //     }
    //     intersections
    // }

    pub fn intersections<'a>(&'a self, ray: Ray) -> Intersections<'a> {
        // Worst case: 2 intersections per object (spheres). Reserve to avoid re-allocs.
        let mut all = Vec::with_capacity(self.objects.len() * 2);

        for obj in &self.objects {
            // If your `Shape::intersect` returns `Intersections<'a>`,
            // prefer an owning extractor; fallback to `all().iter().cloned()`.
            let ints = obj.intersect(ray);

            // Option A (preferred if you add `into_vec()`):
            all.extend(ints.into_vec());

            // Option B (works with your current API):
            // all.extend(ints.all().iter().cloned());
        }

        // Single sort here
        all.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections::from_sorted(all)
    }

    // returns the color at the intersection encapsulated by `comps`
    // in the context of the world
    fn shade_hit(&self, comps: Computations, remaining: i32) -> Color {
        let surface_color = comps.object.material().shade(
            comps.object,
            comps.point,
            self.light,
            comps.eye_vector,
            comps.normal_vector,
            self.is_shadowed(comps.over_point),
        );

        let reflected_color = self.reflected_color(comps, remaining);
        surface_color + reflected_color
    }

    pub fn color_at(&self, ray: Ray, remaining: i32) -> Color {
        // find any intersections the ray makes with the world
        let intersections = self.intersections(ray);

        // and get the first hit
        let hit = intersections.hit();

        match hit {
            Some(hit) => {
                // compute the shading at the intersection point
                let comps = hit.prepare_computations(ray);
                self.shade_hit(comps, remaining)
            }

            // nothing was hit - return BLACK
            None => Color::BLACK,
        }
    }

    // cast a shadow ray from each intersection to the light
    // if something intersects the shadow ray, then the point is in shadow
    pub fn is_shadowed(&self, point: Point) -> bool {
        let vector_to_light = self.light.position - point;
        let distance_to_light = vector_to_light.magnitude();
        let direction_to_light = vector_to_light.normalize();

        let shadow_ray = Ray::new(point, direction_to_light);
        let intersections = self.intersections(shadow_ray);

        if let Some(hit) = intersections.hit() {
            // if the hit is less than the distance to the light then the point is in shadow
            hit.t < distance_to_light
        } else {
            false
        }
    }

    pub fn reflected_color(&self, comps: Computations, remaining: i32) -> Color {
        if remaining <= 0 {
            return Color::BLACK;
        }

        // if the material is not reflective, return BLACK
        if comps.object.material().reflective <= 0.0 {
            return Color::BLACK;
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflect_vector);
        let color = self.color_at(reflect_ray, remaining - 1);
        color * comps.object.material().reflective
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::material::Material;
    use crate::matrix::Transformation;
    use crate::shapes::Plane;
    use crate::shapes::Sphere;
    use crate::utils::EPSILON;
    use crate::vector::Vector;

    use approx::assert_abs_diff_eq;

    // The default world as described in the book - only for testing
    impl Default for World {
        fn default() -> Self {
            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
            let material = Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            };
            let s1 = Shape::from(Sphere::new()).with_material(material);
            let s2 =
                Shape::from(Sphere::new()).with_transform(Transformation::scaling(0.5, 0.5, 0.5));
            let objects = vec![s1, s2];
            Self::new(objects, light)
        }
    }

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
        let sphere1 = Shape::from(Sphere::new()).with_material(material);
        let sphere2 =
            Shape::from(Sphere::new()).with_transform(Transformation::scaling(0.5, 0.5, 0.5));

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
        let color = world.color_at(ray, 5);

        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn color_at_with_intersections() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let color = world.color_at(ray, 5);

        assert_abs_diff_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_when_intersection_behind_ray() {
        let mut world = World::default();
        world.objects[0].material_mut().ambient = 1.0;
        world.objects[1].material_mut().ambient = 1.0;

        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let color = world.color_at(ray, 5);

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

        let s1 = Shape::from(Sphere::new());
        world.objects.push(s1);

        let s2 =
            Shape::from(Sphere::new()).with_transform(Transformation::translation(0.0, 0.0, 10.0));
        world.objects.push(s2.clone());

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &s2);
        let comps = i.prepare_computations(r);
        let c = world.shade_hit(comps, 5);

        assert_abs_diff_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s =
            Shape::from(Sphere::new()).with_transform(Transformation::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &s);
        let comps = i.prepare_computations(r);

        assert_abs_diff_eq!(comps.over_point.z, -EPSILON);
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let world = World::default();
        let ray = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));
        let mut shape = world.objects[1].clone();
        {
            let material = shape.material_mut();
            material.ambient = 1.0;
        }
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(ray);
        let color = world.reflected_color(comps, 0);
        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let world = World::default();
        let shape = Shape::from(Plane::new())
            .with_material(Material {
                reflective: 0.5,
                ..Default::default()
            })
            .with_transform(Transformation::translation(0.0, -1.0, 0.0));
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(ray);
        let color = world.reflected_color(comps, 1);
        assert_abs_diff_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_for_reflective_material() {
        let world = World::default();
        let shape = Shape::from(Plane::new())
            .with_material(Material {
                reflective: 0.5,
                ..Default::default()
            })
            .with_transform(Transformation::translation(0.0, -1.0, 0.0));
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(ray);
        let color = world.shade_hit(comps, 1);
        assert_abs_diff_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut world = World::empty();
        let lower = Shape::from(Plane::new())
            .with_material(Material {
                reflective: 1.0,
                ..Default::default()
            })
            .with_transform(Transformation::translation(0.0, -1.0, 0.0));

        let upper = Shape::from(Plane::new())
            .with_material(Material {
                reflective: 1.0,
                ..Default::default()
            })
            .with_transform(Transformation::translation(0.0, 1.0, 0.0));
        world.objects.push(lower);
        world.objects.push(upper);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        world.color_at(ray, 5);
    }

    #[test]
    fn reflected_color_at_maximum_recursion() {
        let mut world = World::empty();
        let shape = Shape::from(Plane::new())
            .with_material(Material {
                reflective: 1.0,
                ..Default::default()
            })
            .with_transform(Transformation::translation(0.0, -1.0, 0.0));
        world.objects.push(shape);

        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &world.objects[0]);
        let comps = i.prepare_computations(ray);
        let color = world.reflected_color(comps, 0);

        assert_eq!(color, Color::BLACK);
    }
}
