// use crate::Sphere;
use crate::intersection::Intersections;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    transform: Transformation,
    inverse_transform: Transformation, // cached inverse
    material: Material,
    geom: Geometry,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    Sphere(Sphere),
    // Plane(Plane),
}

impl Shape {
    pub fn sphere() -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            material: Material::default(),
            geom: Geometry::Sphere(Sphere::new()),
        }
    }

    fn with_geometry(mut self, g: Geometry) -> Self {
        self.geom = g;
        self
    }

    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self.inverse_transform = t.inverse();
        self
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn with_material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn intersect<'a>(&'a self, ray_world: Ray) -> Intersections<'a> {
        // world -> object space once, here
        let ray_obj = ray_world.transform(self.inverse_transform);
        match &self.geom {
            Geometry::Sphere(s) => Intersections::from_ts(s.local_intersect(ray_obj), self),
            // Geometry::Plane(p) => Intersections::from_ts(p.local_intersect(ray_obj), self),
        }
    }

    pub fn normal_at(&self, p_world: Point) -> Vector {
        // world -> object space once, here
        let p_obj = self.inverse_transform * p_world;

        // ask the geometry for its local-space normal
        let n_obj = match &self.geom {
            Geometry::Sphere(s) => s.local_normal_at(p_obj),
            // Geometry::Plane(p) => p.local_normal_at(p_obj),
        };

        // transform normal back to world space using (inverse^T)
        (self.inverse_transform.transpose() * n_obj).normalize()
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::sphere().with_geometry(Geometry::Sphere(s))
    }
}
