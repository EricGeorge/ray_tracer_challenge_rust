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
    uv_map: Option<fn(Point) -> (f64, f64)>, // function to map points to UV coordinates
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
            uv_map: Some(spherical_map),
        }
    }

    pub fn plane() -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            material: Material::default(),
            geom: Geometry::Plane(Plane::new()),
            uv_map: None,
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

    pub fn uv_map(&self) -> Option<fn(Point) -> (f64, f64)> {
        self.uv_map
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

fn spherical_map(point: Point) -> (f64, f64) {
    //   compute the azimuthal angle
    //   -π < theta <= π
    //   angle increases clockwise as viewed from above,
    //   which is opposite of what we want, but we'll fix it later.
    let theta = point.x.atan2(point.z);

    // vec is the vector pointing from the sphere's origin (the world origin)
    // to the point, which will also happen to be exactly equal to the sphere's
    // radius.
    let vec = Vector::new(point.x, point.y, point.z);
    let radius = vec.magnitude();

    // compute the polar angle
    // 0 <= phi <= π
    let phi = (point.y / radius).acos();

    // -0.5 < raw_u <= 0.5
    let raw_u = theta / (2.0 * std::f64::consts::PI);

    // 0 <= u < 1
    // here's also where we fix the direction of u. Subtract it from 1,
    // so that it increases counterclockwise as viewed from above.
    let u = 1.0 - (raw_u + 0.5);

    // we want v to be 0 at the south pole of the sphere,
    // and 1 at the north pole, so we have to "flip it over"
    // by subtracting it from 1.
    let v = 1.0 - (phi / std::f64::consts::PI);
    (u, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn using_spherical_mapping_on_3d_point() {
        assert_eq!(spherical_map(Point::new(0.0, 0.0, -1.0)), (0.0, 0.5));
        assert_eq!(spherical_map(Point::new(1.0, 0.0, 0.0)), (0.25, 0.5));
        assert_eq!(spherical_map(Point::new(0.0, 0.0, 1.0)), (0.5, 0.5));
        assert_eq!(spherical_map(Point::new(-1.0, 0.0, 0.0)), (0.75, 0.5));
        assert_eq!(spherical_map(Point::new(0.0, 1.0, 0.0)), (0.5, 1.0));
        assert_eq!(spherical_map(Point::new(0.0, -1.0, 0.0)), (0.5, 0.0));
        assert_eq!(
            spherical_map(Point::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)),
            (0.25, 0.75)
        );
    }
}
