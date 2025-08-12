use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::world::World;
use rayon::prelude::*;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

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

    pub fn render(&self, world: &World) -> Canvas
    where
        World: Sync,
    {
        let w = self.hsize;
        let h = self.vsize;
        let total = w * h;

        let camera_inverse = self.transform.inverse();

        let start = Instant::now();
        let pixels: Vec<Color> = (0..total)
            .into_par_iter()
            .map(|i| {
                let x = i % w;
                let y = i / w;

                // Inline, allocation-free ray_for_pixel using precomputed camera_inverse
                let xoffset = (x as f64 + 0.5) * self.pixel_size;
                let yoffset = (y as f64 + 0.5) * self.pixel_size;
                let world_x = self.half_width - xoffset;
                let world_y = self.half_height - yoffset;

                let pixel = camera_inverse * Point::new(world_x, world_y, -1.0);
                let origin = camera_inverse * Point::ORIGIN;
                let direction = (pixel - origin).normalize();
                let ray = Ray::new(origin, direction);

                world.color_at(ray)
            })
            .collect();

        eprintln!("\nDone in {:?}", start.elapsed());

        Canvas {
            width: w,
            height: h,
            pixels,
        }
    }

    pub fn render_with_progress(&self, world: &World) -> Canvas
    where
        World: Sync, // shareable across threads
    {
        let w = self.hsize;
        let h = self.vsize;
        let total = w * h;

        let camera_inverse = self.transform.inverse();

        println!("rendering with {} threads", rayon::current_num_threads());

        let start = Instant::now();
        let progress = AtomicUsize::new(0);
        let step = (total / 100).max(1); // ~1% cadence

        let pixels: Vec<Color> = (0..total)
            .into_par_iter()
            .map(|i| {
                let x = i % w;
                let y = i / w;

                // Inline, allocation-free ray_for_pixel using precomputed camera_inverse
                let xoffset = (x as f64 + 0.5) * self.pixel_size;
                let yoffset = (y as f64 + 0.5) * self.pixel_size;
                let world_x = self.half_width - xoffset;
                let world_y = self.half_height - yoffset;

                let pixel = camera_inverse * Point::new(world_x, world_y, -1.0);
                let origin = camera_inverse * Point::ORIGIN;
                let direction = (pixel - origin).normalize();
                let ray = Ray::new(origin, direction);

                let c = world.color_at(ray);

                // progress display
                let n = progress.fetch_add(1, Ordering::Relaxed) + 1;
                if n % step == 0 || n == total {
                    let pct = 100.0 * (n as f64) / (total as f64);
                    let elapsed = start.elapsed();
                    // \r returns to line start, flush forces immediate update
                    print!("\rProgress: {:>6.2}%  Elapsed Time: {:?}", pct, elapsed);
                    std::io::stdout().flush().unwrap();
                }

                c
            })
            .collect();

        eprintln!("\nDone in {:?}", start.elapsed());
        Canvas::from_pixels(w, h, pixels)
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
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut camera = Camera::new(11, 11, PI / 2.0);
        camera.transform = Matrix::view_transform(
            Point::new(0.0, 0.0, -5.0),
            Point::ORIGIN,
            Vector::new(0.0, 1.0, 0.0),
        );
        let image = camera.render(&w);
        assert_abs_diff_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
