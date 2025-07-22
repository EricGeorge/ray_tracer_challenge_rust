use std::ops;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

fn is_point(t: &Tuple) -> bool {
    t.w == 1.0
}

fn is_vector(t: &Tuple) -> bool {
    t.w == 0.0
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple { x, y, z, w }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        if is_point(&self) && is_point(&other) {
            panic!("Adding two points is not allowed!");
        }
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_as_point() {
        let t = Tuple {
            x: 4.3,
            y: 4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(is_point(&t));
        assert!(!is_vector(&t));
    }

    #[test]
    fn tuple_as_vector() {
        let t = Tuple {
            x: 4.3,
            y: 4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!is_point(&t));
        assert!(is_vector(&t));
    }

    #[test]
    fn create_point() {
        assert_eq!(
            point(4.0, -4.0, 3.0),
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn create_vector() {
        assert_eq!(
            vector(4.0, -4.0, 3.0),
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        );
    }

    #[test]
    fn add_vector_to_point() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);
        assert_eq!(tuple(1.0, 1.0, 6.0, 1.0), p + v);
    }

    #[test]
    fn subtract_point_from_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtract_vector_from_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
    }
}

// TODO - Tests
/*
Scenario: Subtracting a vector from the zero vector
Given zero ← vector(0, 0, 0)
And v ← vector(1, -2, 3)
Then zero - v = vector(-1, 2, -3)

Scenario: Negating a tuple
Given a ← tuple(1, -2, 3, -4)
Then -a = tuple(-1, 2, -3, 4)

Scenario: Multiplying a tuple by a scalar
Given a ← tuple(1, -2, 3, -4)
Then a * 3.5 = tuple(3.5, -7, 10.5, -14)

Scenario: Multiplying a tuple by a fraction
Given a ← tuple(1, -2, 3, -4)
Then a * 0.5 = tuple(0.5, -1, 1.5, -2)

Scenario: Dividing a tuple by a scalar
Given a ← tuple(1, -2, 3, -4)
Then a / 2 = tuple(0.5, -1, 1.5, -2)

Scenario: Computing the magnitude of vector(1, 0, 0)
Given v ← vector(1, 0, 0)
Then magnitude(v) = 1

Scenario: Computing the magnitude of vector(0, 1, 0)
Given v ← vector(0, 1, 0)
Then magnitude(v) = 1

Scenario: Computing the magnitude of vector(0, 0, 1)
Given v ← vector(0, 0, 1)
Then magnitude(v) = 1

Scenario: Computing the magnitude of vector(1, 2, 3)
Given v ← vector(1, 2, 3)
Then magnitude(v) = √14

Scenario: Computing the magnitude of vector(-1, -2, -3)
Given v ← vector(-1, -2, -3)
Then magnitude(v) = √14

Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
Given v ← vector(4, 0, 0)
Then normalize(v) = vector(1, 0, 0)

Scenario: Normalizing vector(1, 2, 3)
Given v ← vector(1, 2, 3)
# vector(1/√ 14, 2/√ 14, 3/√ 14)
Then normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)

Scenario: The magnitude of a normalized vector
Given v ← vector(1, 2, 3)
When norm ← normalize(v)
Then magnitude(norm) = 1

Scenario: The dot product of two tuples
Given a ← vector(1, 2, 3)
And b ← vector(2, 3, 4)
Then dot(a, b) = 20

Scenario: The cross product of two vectors
Given a ← vector(1, 2, 3)
And b ← vector(2, 3, 4)
Then cross(a, b) = vector(-1, 2, -1)
And cross(b, a) = vector(1, -2, 1)

*/
