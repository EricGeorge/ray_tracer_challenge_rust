// use crate::Sphere;
use crate::intersection::Intersection;
use crate::intersection::Intersections;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::ray::Ray;
use crate::shapes::plane::Plane;
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
    Plane(Plane),
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

    pub fn plane() -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            material: Material::default(),
            geom: Geometry::Plane(Plane::new()),
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

    pub fn inverse_transform(&self) -> &Transformation {
        &self.inverse_transform
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
        let ray_obj = ray_world.transform(self.inverse_transform);
        let hits = match &self.geom {
            Geometry::Sphere(s) => s.local_intersect(ray_obj).iter().collect::<Vec<_>>(),
            Geometry::Plane(p) => p.local_intersect(ray_obj).iter().collect::<Vec<_>>(),
        };

        if hits.is_empty() {
            Intersections::empty()
        } else {
            let list = hits
                .into_iter()
                .map(|t| Intersection { t, s: self })
                .collect::<Vec<_>>();
            Intersections::from_sorted(list)
        }
    }

    pub fn normal_at(&self, p_world: Point) -> Vector {
        // world -> object space once, here
        let p_obj = self.inverse_transform * p_world;

        // ask the geometry for its local-space normal
        let n_obj = match &self.geom {
            Geometry::Sphere(s) => s.local_normal_at(p_obj),
            Geometry::Plane(p) => p.local_normal_at(p_obj),
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

impl From<Plane> for Shape {
    fn from(s: Plane) -> Self {
        Shape::plane().with_geometry(Geometry::Plane(s))
    }
}
