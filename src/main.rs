use image::{ImageBuffer, Rgba};
use rand::Rng;
use std::path::Path;

/// Applies various filters and effects to an input image and saves the result.
///
/// # Arguments
///
/// * `input_image_path` - The path to the input image file.
/// * `output_image_path` - The path where the processed image will be saved.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) if successful, or an error if something goes wrong.
fn apply_filter(input_image_path: &str, output_image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input image
    let img = image::open(&Path::new(input_image_path))?;
    let mut filtered_img = img.to_rgba8();

    // Apply grain effect
    add_grain(&mut filtered_img);

    // Enhance colors using a more subtle technique
    let enhanced_img = enhance_colors(&filtered_img);

    // Add a very subtle glow effect
    let glowed_img = add_glow(&enhanced_img);

    // Apply a subtle sharpening
    let final_img = sharpen(&glowed_img);

    // Save the filtered image
    final_img.save(&Path::new(output_image_path))?;
    Ok(())
}

/// Adds a grain effect to the image by introducing random noise.
///
/// # Arguments
///
/// * `img` - A mutable reference to the image buffer.
fn add_grain(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut rng = rand::thread_rng();
    for pixel in img.pixels_mut() {
        let noise: i16 = rng.gen_range(-10..=10);
        for c in 0..3 {
            pixel[c] = ((pixel[c] as i16 + noise).max(0).min(255)) as u8;
        }
    }
}

/// Enhances colors using a more subtle technique.
///
/// # Arguments
///
/// * `img` - The input image buffer.
///
/// # Returns
///
/// * An `ImageBuffer` with slightly enhanced colors.
fn enhance_colors(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut enhanced_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in enhanced_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        for c in 0..3 {
            let value = original[c] as f32;
            pixel[c] = ((value * 1.05).min(255.0)) as u8;
        }
        pixel[3] = original[3]; // Preserve alpha channel
    }

    enhanced_img
}

/// Adds a very subtle glow effect to the image.
///
/// # Arguments
///
/// * `img` - The input image buffer.
///
/// # Returns
///
/// * An `ImageBuffer` with a subtle glow effect applied.
fn add_glow(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut glowed_img = img.clone();
    let glow_radius = 3;

    for y in glow_radius..height - glow_radius {
        for x in glow_radius..width - glow_radius {
            let mut glow = [0.0; 3];
            for dy in -(glow_radius as i32)..=(glow_radius as i32) {
                for dx in -(glow_radius as i32)..=(glow_radius as i32) {
                    let pixel = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                    let weight = 1.0 / ((dx * dx + dy * dy) as f32 + 1.0);
                    for c in 0..3 {
                        glow[c] += pixel[c] as f32 * weight;
                    }
                }
            }
            let pixel = glowed_img.get_pixel_mut(x, y);
            for c in 0..3 {
                pixel[c] = ((pixel[c] as f32 * 0.95 + glow[c] * 0.05).min(255.0)) as u8;
            }
        }
    }

    glowed_img
}

/// Sharpens the image using a simple convolution kernel.
///
/// # Arguments
///
/// * `img` - The input image buffer.
///
/// # Returns
///
/// * An `ImageBuffer` with slightly increased sharpness.
fn sharpen(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut sharpened_img = img.clone();

    let kernel: [[f32; 3]; 3] = [
        [-0.1, -0.1, -0.1],
        [-0.1,  1.8, -0.1],
        [-0.1, -0.1, -0.1],
    ];

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut new_pixel = [0.0; 4];
            for ky in 0..3 {
                for kx in 0..3 {
                    let pixel = img.get_pixel(x + kx - 1, y + ky - 1);
                    for c in 0..3 {
                        new_pixel[c] += pixel[c] as f32 * kernel[ky as usize][kx as usize];
                    }
                }
            }
            let output_pixel = sharpened_img.get_pixel_mut(x, y);
            for c in 0..3 {
                output_pixel[c] = new_pixel[c].max(0.0).min(255.0) as u8;
            }
            output_pixel[3] = img.get_pixel(x, y)[3]; // Preserve original alpha
        }
    }

    sharpened_img
}

fn main() {
    let input_image_path: &str = "src/input.png";
    let output_image_path: &str = "src/output.png";

    // Check if the input file exists
    if !Path::new(input_image_path).exists() {
        println!("Error: Input file '{}' not found.", input_image_path);
        println!("Please make sure the input image is in the same directory as the executable.");
        return;
    }

    match apply_filter(input_image_path, output_image_path) {
        Ok(_) => println!("Image processing completed successfully."),
        Err(e) => println!("Error processing image: {}", e),
    }
}