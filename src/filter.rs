use image::{ImageBuffer, Rgba, RgbaImage};

fn clamp_color(color: f64) -> u8 {
    match color {
        c if  c < 0.0 => 0,
        c if c > 255.0 => 255,
        _ => color as u8  
    }
}


fn blur_kernel(size: usize) -> Vec<f64> {
    let side = 2 * size + 1;
    let length = side * side; 
    let kernel  = vec![1.0 / (length as f64); length];
    kernel
}

fn sobel_kernel(size: usize) -> Vec<f64> {
    let side = size * 2 + 1;
    let length = side * side;
    let center = length / 2;
    let mut matrix = vec![1.0 /  side as f64 ; length]; 

    matrix[center] = -(length as f64 / side as f64);

    matrix
}


fn convole(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: Vec<f64>) -> RgbaImage {

    let (width, height) = image.dimensions();
    let size = (kernel.len() as f64).sqrt() as i32;
    let side = (size - 1) / 2;  

    let mut convole_image = RgbaImage::new(width, height);
    for x in 0..width {
        for y in 0..height {

            let pixel = image.get_pixel(x, y);
            let mut red = 0.0;

            for dx in -side..=side {
                for dy in -side..=side {
                    let sx = dx + (x as i32 );
                    let sy =  dy + (y as i32);

                    if sy >= 0 && sy < (height as i32) && sx >=0 && sx < (width as i32) {
                        let kernel_index = dx + side + (dy + side) * size;
                        red += (image.get_pixel(sx as u32, sy as u32)[0] as f64) * kernel[kernel_index as usize]; 
                    }
                }
            }

            let new_pixel = Rgba::from([clamp_color(red), pixel[1], pixel[2], pixel[3]]);
            convole_image.put_pixel(x, y, new_pixel);


        }
    }
    convole_image
}


pub fn blur_filter(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, size: usize) -> RgbaImage {
    let kernel = blur_kernel(size);
    convole(image, kernel)
}

pub fn sobel_filter(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, size: usize) -> RgbaImage {
    let kernel = sobel_kernel(size);
    convole(image, kernel)
}

#[cfg(test)]
mod test {
    use image::{Rgba, RgbaImage};

    use crate::filter::convole;

    use super::{blur_filter, sobel_filter};


    #[test]
    fn test_convole() {
        // generate a test image 
        let image = generate_test_image();
    
        // create a test kernel
        let kernel = vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];

        // apply the convolution
        let result = convole(&image, kernel);

        // check pixel
        assert_eq!(*result.get_pixel(0, 0), Rgba::from([255, 255, 255, 255]));
        assert_eq!(*result.get_pixel(1, 0), Rgba::from([0, 0, 0, 255]));
        assert_eq!(*result.get_pixel(2, 0), Rgba::from([255, 255, 255, 255]));
        assert_eq!(*result.get_pixel(0, 1), Rgba::from([255, 0, 0, 255]));
        assert_eq!(*result.get_pixel(1, 1), Rgba::from([255, 0, 0, 255]));
        assert_eq!(*result.get_pixel(2, 1), Rgba::from([255, 0, 0, 255]));
        assert_eq!(*result.get_pixel(0, 2), Rgba::from([255, 255, 255, 255]));
        assert_eq!(*result.get_pixel(1, 2), Rgba::from([0, 0, 0, 255]));
    }



    #[test]
    fn test_blur_filter() {
        // generate a test image 
        let image = generate_test_image();

        // apply the blurred filter
        let blurred_image = blur_filter(&image, 1);

        // check the dimensions
        assert_eq!(image.width(), blurred_image.width());
        assert_eq!(image.height(), blurred_image.height());
    
    }

    #[test]
    fn test_sobel_filter() {
        // generate a test image 
        let image = generate_test_image();

        // apply the blurred filter
        let filtered_image = sobel_filter(&image, 3);

        // check the dimensions
        assert_eq!(image.width(), filtered_image.width());
        assert_eq!(image.height(), filtered_image.height());
    }

    fn generate_test_image() -> RgbaImage {
        let mut image = RgbaImage::new(3, 3);
        image.put_pixel(0, 0, Rgba::from([255, 255, 255, 255]));
        image.put_pixel(1, 0, Rgba::from([0, 0, 0, 255]));
        image.put_pixel(2, 0, Rgba::from([255, 255, 255, 255]));
        image.put_pixel(0, 1, Rgba::from([255, 0, 0, 255]));
        image.put_pixel(1, 1, Rgba::from([255, 0, 0, 255]));
        image.put_pixel(2, 1, Rgba::from([255, 0, 0, 255]));
        image.put_pixel(0, 2, Rgba::from([255, 255, 255, 255]));
        image.put_pixel(1, 2, Rgba::from([0, 0, 0, 255]));
        image.put_pixel(2, 2, Rgba::from([255, 255, 255, 255]));

        image
    }


    #[test]
    fn test_perroquet() {
        // generate a test image 
        let image = image::open("perroquet.jpeg").unwrap();
        let rgb_image = image.to_rgba8();

        // apply the blurred filter
        let filtered_image = sobel_filter(&rgb_image, 6);

        filtered_image.save("perroquet_edge.jpeg").unwrap();

        // check the dimensions
        assert_eq!(image.width(), filtered_image.width());
        assert_eq!(image.height(), filtered_image.height());
    }
}
