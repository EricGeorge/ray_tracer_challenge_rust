use super::canvas::Canvas;
use super::matrix::Matrix;
use super::point::Point;
use super::ray::Ray;
use super::world::World;

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix<4>,
    pub pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let transform = Matrix::identity();

        // the camera's canvas is always one unit in front of the camera
        // this helps with math a little
        // calculate the pixel size (in world-space units)
        // use a right triangle to calculate half of the field of view
        let half_view = (field_of_view / 2.0).tan();

        // depending on the aspect ration, half_view is either half the width or height
        let aspect = hsize as f64 / vsize as f64;
        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };
        Self {
            hsize,
            vsize,
            field_of_view,
            transform,
            pixel_size: half_width * 2.0 / hsize as f64,
            half_width,
            half_height,
        }
    }

    // calculate the ray that starts at the camera and passes through the point on the canvas
    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::ORIGIN;
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    // find the color for every pixel in the canvas
    // Note the on_progress closure is for callers that may implement progress feedback
    pub fn render<F>(&self, world: &World, mut on_progress: F) -> Canvas
    where
        F: FnMut(),
    {
        let mut image = Canvas::empty(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write_pixel(x, y, color);

                on_progress()
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::point::Point;
    use crate::vector::Vector;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn create_camera() {
        let camera = Camera::new(160, 120, PI / 2.0);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_abs_diff_eq!(camera.field_of_view, PI / 2.0);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.0);

        assert_abs_diff_eq!(camera.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.0);

        assert_abs_diff_eq!(camera.pixel_size, 0.01);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(100, 50);

        assert_abs_diff_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(ray.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_for_corner_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(0, 0);
        assert_abs_diff_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(ray.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_for_transformed_camera() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.transform = Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
        let ray = camera.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(ray.origin, Point::new(0.0, 2.0, -5.0));
        assert_abs_diff_eq!(
            ray.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut camera = Camera::new(11, 11, PI / 2.0);
        camera.transform = Matrix::view_transform(
            Point::new(0.0, 0.0, -5.0),
            Point::ORIGIN,
            Vector::new(0.0, 1.0, 0.0),
        );
        let image = camera.render(&w, || {});
        assert_abs_diff_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
