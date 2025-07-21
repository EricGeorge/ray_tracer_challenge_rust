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
