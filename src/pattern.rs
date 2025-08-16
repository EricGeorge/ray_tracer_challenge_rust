// FUTURE TODO:  Add support for blended patterns, more UV patterns, and noise jittered patterns

use crate::color::Color;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::shapes::Shape;

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Solid(Color),
    Pattern(Box<Pattern>),
}

impl From<Color> for Source {
    fn from(c: Color) -> Self {
        Source::Solid(c)
    }
}
impl From<Pattern> for Source {
    fn from(p: Pattern) -> Self {
        Source::Pattern(Box::new(p))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    Striped,
    Gradient,
    Ring,
    Checker,
    CheckerUV { width: f64, height: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    transform: Transformation,
    inverse_transform: Transformation,
    pattern_type: PatternType,
    a: Source,
    b: Source,
}

impl Pattern {
    pub fn striped<A: Into<Source>, B: Into<Source>>(a: A, b: B) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Striped,
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn gradient<A: Into<Source>, B: Into<Source>>(a: A, b: B) -> Self {
        /* same shape */
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Gradient,
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn ring<A: Into<Source>, B: Into<Source>>(a: A, b: B) -> Self {
        /* ... */
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Ring,
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn checker<A: Into<Source>, B: Into<Source>>(a: A, b: B) -> Self {
        /* ... */
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Checker,
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn checker_uv<A: Into<Source>, B: Into<Source>>(
        width: f64,
        height: f64,
        a: A,
        b: B,
    ) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::CheckerUV { width, height },
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self.inverse_transform = t.inverse();
        self
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn with_type(mut self, pattern_type: PatternType) -> Self {
        self.pattern_type = pattern_type;
        self
    }
}

impl Pattern {
    fn sample_source(&self, src: &Source, local_pt: Point, object: &Shape) -> Color {
        match src {
            Source::Solid(c) => *c,
            Source::Pattern(p) => p.pattern_at_object(object, local_pt),
        }
    }
}

const EPS_PLANE: f64 = 1e-6; // near-planar threshold for |y|
const EPS_FLOOR: f64 = 1e-9; // bias to avoid underflow right below integers

#[inline]
fn floor_eps(x: f64) -> i32 {
    // Bias toward the “current” cell so x = 1.000000000 - 1e-15 doesn’t floor to 0
    let bias = if x >= 0.0 { EPS_FLOOR } else { -EPS_FLOOR };
    (x + bias).floor() as i32
}

impl Pattern {
    pub fn pattern_at_object(&self, object: &Shape, point: Point) -> Color {
        let object_point = *object.inverse_transform() * point;
        let pattern_point = self.inverse_transform * object_point;

        match &self.pattern_type {
            PatternType::Striped => self.stripe_at(pattern_point, object),
            PatternType::Gradient => self.gradient_at(pattern_point, object),
            PatternType::Ring => self.ring_at(pattern_point, object),
            PatternType::Checker => self.checker_at(pattern_point, object),
            PatternType::CheckerUV { width, height } => {
                self.checker_uv_at(pattern_point, object, *width, *height)
            }
        }
    }

    fn stripe_at(&self, p: Point, obj: &Shape) -> Color {
        if (p.x.floor() as i32) % 2 == 0 {
            self.sample_source(&self.a, p, obj)
        } else {
            self.sample_source(&self.b, p, obj)
        }
    }

    fn gradient_at(&self, p: Point, obj: &Shape) -> Color {
        let ca = self.sample_source(&self.a, p, obj);
        let cb = self.sample_source(&self.b, p, obj);
        let t = p.x - p.x.floor();
        ca + (cb - ca) * t
    }

    fn ring_at(&self, p: Point, obj: &Shape) -> Color {
        let r = p.x.hypot(p.z);
        if (r.floor() as i32) % 2 == 0 {
            self.sample_source(&self.a, p, obj)
        } else {
            self.sample_source(&self.b, p, obj)
        }
    }

    fn checker_at(&self, p: Point, obj: &Shape) -> Color {
        // If we’re effectively on a plane (|y| tiny in local/pattern space), drop y from parity.
        // This avoids flicker from y ≈ ±0 and microscopic negatives.
        let ix = floor_eps(p.x);
        let iz = floor_eps(p.z);

        let s = if p.y.abs() < EPS_PLANE {
            ix + iz
        } else {
            ix + floor_eps(p.y) + iz
        };

        if s % 2 == 0 {
            self.sample_source(&self.a, p, obj)
        } else {
            self.sample_source(&self.b, p, obj)
        }
    }

    fn checker_uv_at(&self, p: Point, obj: &Shape, width: f64, height: f64) -> Color {
        if let Some(uv_fn) = obj.uv_map() {
            let (u, v) = uv_fn(p);

            // Clamp slightly inside [0,1) to avoid landing exactly on the top/right edge.
            let u = u.clamp(0.0, 1.0 - EPS_FLOOR) * width;
            let v = v.clamp(0.0, 1.0 - EPS_FLOOR) * height;

            let ix = floor_eps(u);
            let iy = floor_eps(v);

            if (ix + iy) % 2 == 0 {
                self.sample_source(&self.a, p, obj)
            } else {
                self.sample_source(&self.b, p, obj)
            }
        } else {
            // Fallback if no uv_map
            self.sample_source(&self.a, p, obj)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::matrix::Transformation;
    use crate::shapes::Shape;
    use crate::shapes::Sphere;

    use super::*;

    #[test]
    fn create_a_stripe_pattern() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.a, Source::Solid(Color::WHITE));
        assert_eq!(pattern.b, Source::Solid(Color::BLACK));
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());
        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 1.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 2.0, 0.0), &sphere),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 0.0, 1.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 0.0, 2.0), &sphere),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.stripe_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(0.9, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(Point::new(1.0, 0.0, 0.0), &sphere),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(Point::new(-0.1, 0.0, 0.0), &sphere),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(Point::new(-1.0, 0.0, 0.0), &sphere),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(Point::new(-1.1, 0.0, 0.0), &sphere),
            Color::WHITE
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        let object = Shape::from(Sphere::new())
            .with_material(Material {
                pattern: Some(pattern.clone()),
                ..Default::default()
            })
            .with_transform(Transformation::scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn stripes_with_an_pattern_transformation() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK)
            .with_transform(Transformation::scaling(2.0, 2.0, 2.0));
        let object = Shape::from(Sphere::new()).with_material(Material {
            pattern: Some(pattern.clone()),
            ..Default::default()
        });
        let c = pattern.pattern_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_pattern_transformation() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK)
            .with_transform(Transformation::scaling(2.0, 2.0, 2.0));

        let object = Shape::from(Sphere::new())
            .with_material(Material {
                pattern: Some(pattern.clone()),
                ..Default::default()
            })
            .with_transform(Transformation::scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_object(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn gradient_pattern() {
        let pattern = Pattern::gradient(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.gradient_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.gradient_at(Point::new(0.25, 0.0, 0.0), &sphere),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.gradient_at(Point::new(0.5, 0.0, 0.0), &sphere),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.gradient_at(Point::new(0.75, 0.0, 0.0), &sphere),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_test() {
        let pattern = Pattern::ring(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.ring_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.ring_at(Point::new(1.0, 0.0, 0.0), &sphere),
            Color::BLACK
        );
        assert_eq!(
            pattern.ring_at(Point::new(0.0, 0.0, 1.0), &sphere),
            Color::BLACK
        );
        assert_eq!(
            pattern.ring_at(Point::new(0.708, 0.0, 0.708), &sphere),
            Color::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(0.99, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(1.01, 0.0, 0.0), &sphere),
            Color::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.99, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(0.0, 1.01, 0.0), &sphere),
            Color::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        let sphere = Shape::from(Sphere::new());

        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.0, 0.0), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.0, 0.99), &sphere),
            Color::WHITE
        );
        assert_eq!(
            pattern.checker_at(Point::new(0.0, 0.0, 1.01), &sphere),
            Color::BLACK
        );
    }

    // #[test]
    // fn checker_pattern_in_2d() {
    //     let pattern = Pattern::checker_uv(2.0, 2.0, Color::BLACK, Color::WHITE);
    //     let sphere = Shape::from(Sphere::new());
    //     assert_eq!(pattern.checker_uv_at(0.0, 0.0), Color::BLACK);
    //     assert_eq!(pattern.checker_uv_pattern_at(0.5, 0.0), Color::WHITE);
    //     assert_eq!(pattern.checker_uv_pattern_at(0.0, 0.5), Color::WHITE);
    //     assert_eq!(pattern.checker_uv_pattern_at(0.5, 0.5), Color::BLACK);
    //     assert_eq!(pattern.checker_uv_pattern_at(1.0, 1.0), Color::BLACK);
    // }
    #[test]
    fn using_texture_map_pattern_with_spherical_map() {
        let pattern = Pattern::checker_uv(16.0, 8.0, Color::BLACK, Color::WHITE);
        let sphere = Shape::from(Sphere::new());
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(0.4315, 0.4670, 0.7719)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(-0.9654, 0.2552, -0.0534)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(0.1039, 0.7090, 0.6975)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(-0.4986, -0.7856, -0.3663)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(-0.0317, -0.9395, 0.3411)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(0.4809, -0.7721, 0.4154)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(0.0285, -0.9612, -0.2745)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(-0.5734, -0.2162, -0.7903)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(0.7688, -0.1470, 0.6223)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at_object(&sphere, Point::new(-0.7652, 0.2175, 0.6060)),
            Color::BLACK
        );
    }
}
