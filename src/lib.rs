use image::{Rgba, RgbaImage, DynamicImage, ImageBuffer, Pixel};
use rand::Rng;

use crate::delaunay::*;
use crate::filter::*;


pub mod filter;
pub mod delaunay;
pub mod drawer;
pub mod cli;



pub struct Triangulation {
    pub blur_factor: usize,
    pub sobel_factor: usize,
    pub max_points: usize,
    pub points_threshold: i32,
    pub point_rate: f64, 
    pub grayscale: bool,
}

impl Default for Triangulation {

    fn default() -> Self {
        Self { 
            blur_factor: 1, 
            sobel_factor: 6,
            points_threshold: 10, 
            grayscale: false, 
            max_points: 2500,
            point_rate: 0.075
        }
    }
}


impl Triangulation {


    pub fn generate_triangle(&self,  image: DynamicImage) -> (Vec<Triangle>, RgbaImage) {
        let mut source_image = image.to_rgba8();
        let (width, height) = source_image.dimensions();
        if self.grayscale {
            source_image = image.grayscale().to_rgba8();
        } 

        let blur_image = blur_filter(&source_image, self.blur_factor);
        let edge_image = sobel_filter(&blur_image, self.sobel_factor);
        let points = self.get_points(&edge_image);
        let mut delonay = Delaunay::new(height as f64, width as f64);
        delonay.add_points(points);

        let triangles = delonay.triangles();
        (triangles, source_image)
    }

    fn get_points(&self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        let mut points: Vec<Point> = vec![];
        let mut dpoints: Vec<Point> = vec![];
        let (width, height) = image.dimensions();

        for  x in 0..width {
            for y in 0..height {
                let (mut sum, mut total) = (0, 0);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let sx = (x as i32) + dx;
                        let sy = (y as i32) + dy;
                        if sx >=0  && sx < width as i32 && sy >=0 && sy < height as i32 {
                            total += 1; 
                            let red = image.get_pixel(sx as u32, sy as u32).channels()[0];
                            sum += red as i32
                        }
                    }
                }

                let avg = if total > 0 { sum / total } else { 0 };
                if avg > self.points_threshold {
                    points.push(Point { x: x as f64, y: y as f64 });
                }
            }
        }

        let limit = (points.len() as f64 * self.point_rate).min(self.max_points as f64) as usize;
        for _ in 0..limit {
            let j = rng.gen_range(0..points.len());
            dpoints.push(points[j]);
        }
        return  dpoints;
    }
}

#[cfg(test)]
mod test {
    use image::{Rgba, RgbaImage};
    use rand::Rng;
    use super::Triangulation;

    #[test]
    fn test_get_points() {
        // create a random image 
        let mut rng = rand::thread_rng();
        let mut img = RgbaImage::new(10, 10);
        for x in 0..img.width() {
            for y in 0..img.height() {
                let pixel = Rgba::from([ rng.gen_range(0..255) as u8 , rng.gen_range(0..255) as u8 , rng.gen_range(0..255) as u8, 255]);
                img.put_pixel(x, y, pixel)
            }
        }

        // Test with different thresholds and max_points values
        for &threshold in &[50, 100, 150] {
            for &max_points in &[5, 10, 20] {
                let triangulation = Triangulation {
                    points_threshold: threshold,
                    max_points: max_points as usize,
                    ..Default::default()
                };
                let points = triangulation.get_points(&img);
                
                // Check that the number of returned points is within the expected range
                assert!(points.len() <= max_points);

                // Check all point are valid
                points.iter().for_each(|point| {
                    assert!(point.x < (img.width() as f64));
                    assert!(point.y < (img.height() as f64));
                })
            }
        }
    }
}
