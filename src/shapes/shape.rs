use super::sphere::Sphere;
use crate::color::Color;
use crate::intersection::Intersections;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::point_light::PointLight;
use crate::ray::Ray;
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
                // get raw t’s
                let ts = s.local_intersect(ray_obj);
                // wrap into Intersections that borrow `&Shape`
                Intersections::from_ts(ts, self)
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
                // transform normal back to world space with inverse-transpose
                (inv.transpose() * s.local_normal_at(p_obj)).normalize()
            } // Shape::Plane(p) => {
              //     let inv = p.inverse_transform;
              //     let p_obj = inv * p_world;
              //     (inv.transpose() * p.local_normal_at(p_obj)).normalize()
              // }
        }
    }

    pub fn lighting(
        &self,
        position: Point,
        light: PointLight,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        match self {
            Shape::Sphere(s) => s.lighting(position, light, eye, normal, in_shadow),
            // Shape::Plane(p) => p.lighting(position, light, eye, normal, in_shadow),
        }
    }

    pub fn with_transform(self, t: Transformation) -> Self {
        match self {
            Shape::Sphere(s) => Shape::Sphere(s.with_transform(t)),
            // Shape::Plane(p) => Shape::Plane(p.with_transform(t)),
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
