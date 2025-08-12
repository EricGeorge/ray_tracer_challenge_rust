use crate::color::Color;
use crate::matrix::Transformation;
use crate::point::Point;
use crate::shapes::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    transform: Transformation,
    inverse_transform: Transformation,
    a: Color,
    b: Color,
}

impl Pattern {
    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self.inverse_transform = t.inverse();
        self
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn striped(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transformation::identity(),
            inverse_transform: Transformation::identity(),
        }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &Shape, point: Point) -> Color {
        let object_point = object.transform().inverse() * point;
        let pattern_point = self.transform().inverse() * object_point;

        self.stripe_at(pattern_point)
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
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));
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
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));
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

        let c = pattern.stripe_at_object(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }
}
