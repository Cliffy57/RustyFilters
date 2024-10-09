use image::{ImageBuffer, Rgba};
use rand::prelude::*;
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

pub fn apply_filter<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    grain_intensity: i16,
    color_enhancement: f32,
    glow_intensity: f32,
    sharpness: f32,
    exposure : f32, 
    apply_grayscale: bool, // New parameter to control grayscale filter
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let mut filtered_img = img.to_rgba8();

    if apply_grayscale {
        filtered_img = to_grayscale(&filtered_img);
    }

    add_grain(&mut filtered_img, grain_intensity);
    let enhanced_img = enhance_colors(&filtered_img, color_enhancement);
    let glowed_img = add_glow(&enhanced_img, glow_intensity);
    let exposed_img = adjust_exposure(&glowed_img, exposure);
    let final_img = sharpen(&exposed_img, sharpness);

    final_img.save(output_path)?;
    Ok(())
}

/// Adds a grain effect to the image by introducing random noise.
///
/// # Arguments
///
/// * `img` - A mutable reference to the image buffer.
/// * `intensity` - The intensity of the grain effect.
fn add_grain(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, intensity: i16) {
    let mut rng = rand::thread_rng();
    for pixel in img.pixels_mut() {
        let noise: i16 = rng.gen_range(-intensity..=intensity);
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
fn enhance_colors(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    enhancement: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut enhanced_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in enhanced_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        for c in 0..3 {
            let value = original[c] as f32;
            pixel[c] = ((value * enhancement).min(255.0)) as u8;
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
fn add_glow(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    intensity: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
                pixel[c] =
                    ((pixel[c] as f32 * (1.0 - intensity) + glow[c] * intensity).min(255.0)) as u8;
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

fn sharpen(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, sharpness: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut sharpened_img = img.clone();

    let center = 1.0 + 4.0 * sharpness;
    let sides = -sharpness;
    let kernel: [[f32; 3]; 3] = [[0.0, sides, 0.0], [sides, center, sides], [0.0, sides, 0.0]];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
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
/// Converts the image to grayscale.
///
/// # Arguments
///
/// * `img` - The input image buffer.
///
/// # Returns
///
/// * An `ImageBuffer` with the grayscale effect applied.
fn to_grayscale(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut grayscale_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in grayscale_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        let gray_value = (0.299 * original[0] as f32
            + 0.587 * original[1] as f32
            + 0.114 * original[2] as f32) as u8;
        for c in 0..3 {
            pixel[c] = gray_value;
        }
        pixel[3] = original[3]; // Preserve alpha channel
    }

    grayscale_img
}

/// Adjusts the exposure of the image.
///
/// # Arguments
///
/// * `img` - The input image buffer.
/// * `adjustment` - The exposure adjustment factor. Positive values increase exposure, negative values decrease exposure.
///
/// # Returns
///
/// * An `ImageBuffer` with the exposure adjusted.
fn adjust_exposure(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    adjustment: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut adjusted_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in adjusted_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        for c in 0..3 {
            let value = original[c] as f32;
            pixel[c] = ((value * adjustment).min(255.0).max(0.0)) as u8;
        }
        pixel[3] = original[3]; // Preserve alpha channel
    }
    adjusted_img
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

    match apply_filter(input_image_path, output_image_path, 20, 0.5, 0.2, 0.8,1.0, true) {
        Ok(_) => println!("Image processing completed successfully."),
        Err(e) => println!("Error processing image: {}", e),
    }
}
