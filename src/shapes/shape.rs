// use crate::Sphere;
use crate::intersection::Intersections;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Sphere(Sphere),
    // Plane(Plane),
}

impl Shape {
    pub fn intersect<'a>(&'a self, ray_world: Ray) -> Intersections<'a> {
        match self {
            Shape::Sphere(s) => {
                // world -> object space for this shape
                let ray_obj = ray_world.transform(*s.inverse_transform());
                Intersections::from_ts(s.local_intersect(ray_obj), self)
            } // Shape::Plane(p) => {
              //     let ray_obj = ray_world.transform(p.inverse_transform);
              //     let ts = p.local_intersect(ray_obj);
              //     Intersections::from_ts(ts, self)
              // }
        }
    }

    pub fn normal_at(&self, p_world: Point) -> Vector {
        match self {
            Shape::Sphere(s) => {
                let inv = *s.inverse_transform();
                let p_obj = inv * p_world;

                // Note:  normal vectors are transformed using the transpose
                // of the inverse of the transformation matrix, because this ensures
                // that the transformed normal vector remains perpendicular to the transformed surface.
                (inv.transpose() * s.local_normal_at(p_obj)).normalize()
            } // Shape::Plane(p) => {
              //     let inv = p.inverse_transform;
              //     let p_obj = inv * p_world;
              //     (inv.transpose() * p.local_normal_at(p_obj)).normalize()
              // }
        }
    }

    pub fn with_transform(self, t: Transformation) -> Self {
        match self {
            Shape::Sphere(s) => Shape::Sphere(s.with_transform(t)),
            // Shape::Plane(p) => Shape::Plane(p.with_transform(t)),
        }
    }

    pub fn transform(&self) -> &Transformation {
        match self {
            Shape::Sphere(s) => s.transform(),
            // Shape::Plane(p) => p.transform(),
        }
    }

    pub fn with_material(self, m: Material) -> Self {
        match self {
            Shape::Sphere(s) => Shape::Sphere(s.with_material(m)),
            // Shape::Plane(p) => Shape::Plane(p.with_material(m)),
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => s.material(),
            // Shape::Plane(p) => p.material(),
        }
    }

    pub fn material_mut(&mut self) -> &mut Material {
        match self {
            Shape::Sphere(s) => s.material_mut(),
            // Shape::Plane(p) => p.material_mut(),
        }
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
