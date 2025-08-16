use crate::color::Color;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::shapes::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    transform: Transformation,
    inverse_transform: Transformation,
    pattern_type: PatternType,
    a: Color,
    b: Color,
    width: f64,
    height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    Striped,
    Gradient,
    Ring,
    Checker,
    CheckerUV,
}

impl Pattern {
    pub fn striped(a: Color, b: Color) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Striped,
            a,
            b,
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Gradient,
            a,
            b,
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Ring,
            a,
            b,
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn checker(a: Color, b: Color) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::Checker,
            a,
            b,
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn checker_uv(width: f64, height: f64, a: Color, b: Color) -> Self {
        Self {
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
            pattern_type: PatternType::CheckerUV,
            a,
            b,
            width,
            height,
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

    pub fn pattern_at_object(&self, object: &Shape, point: Point) -> Color {
        let object_point = *object.inverse_transform() * point;
        let pattern_point = self.inverse_transform * object_point;

        match self.pattern_type {
            PatternType::Striped => self.stripe_at(pattern_point),
            PatternType::Gradient => self.gradient_at(pattern_point),
            PatternType::Ring => self.ring_at(pattern_point),
            PatternType::Checker => self.checker_at(pattern_point),
            PatternType::CheckerUV => self.checker_uv_at(pattern_point, object.uv_map()),
        }
    }

    fn stripe_at(&self, point: Point) -> Color {
        if (point.x.floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn gradient_at(&self, point: Point) -> Color {
        let fraction = point.x - point.x.floor();
        let distance = self.b - self.a;
        self.a + distance * fraction
    }

    fn ring_at(&self, point: Point) -> Color {
        let distance = point.x.hypot(point.z);
        if distance.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn checker_at(&self, point: Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn checker_uv_at(&self, point: Point, uv_map: Option<fn(Point) -> (f64, f64)>) -> Color {
        if let Some(uv_fn) = uv_map {
            let (u, v) = uv_fn(point);
            self.checker_uv_pattern_at(u, v)
        } else {
            // Fallback: return color a if no uv_map is provided
            self.a
        }
    }

    fn checker_uv_pattern_at(&self, u: f64, v: f64) -> Color {
        if ((u * self.width).floor() as i32 + (v * self.height).floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
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
        assert_eq!(pattern.a, Color::WHITE);
        assert_eq!(pattern.b, Color::BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), Color::WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), Color::WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::striped(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), Color::WHITE);
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
        assert_eq!(pattern.gradient_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(
            pattern.gradient_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.gradient_at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.gradient_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_test() {
        let pattern = Pattern::ring(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.ring_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.ring_at(Point::new(1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.ring_at(Point::new(0.0, 0.0, 1.0)), Color::BLACK);
        assert_eq!(pattern.ring_at(Point::new(0.708, 0.0, 0.708)), Color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(0.99, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(1.01, 0.0, 0.0)), Color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.99, 0.0)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(0.0, 1.01, 0.0)), Color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checker(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.0, 0.99)), Color::WHITE);
        assert_eq!(pattern.checker_at(Point::new(0.0, 0.0, 1.01)), Color::BLACK);
    }

    #[test]
    fn checker_pattern_in_2d() {
        let pattern = Pattern::checker_uv(2.0, 2.0, Color::BLACK, Color::WHITE);
        assert_eq!(pattern.checker_uv_pattern_at(0.0, 0.0), Color::BLACK);
        assert_eq!(pattern.checker_uv_pattern_at(0.5, 0.0), Color::WHITE);
        assert_eq!(pattern.checker_uv_pattern_at(0.0, 0.5), Color::WHITE);
        assert_eq!(pattern.checker_uv_pattern_at(0.5, 0.5), Color::BLACK);
        assert_eq!(pattern.checker_uv_pattern_at(1.0, 1.0), Color::BLACK);
    }

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
