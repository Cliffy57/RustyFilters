use image::{ImageBuffer, Rgba};
use rand::prelude::*;
use std::path::{Path, PathBuf};

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

pub fn apply_filter(
    input_path: &PathBuf,
    output_path: &PathBuf,
    grain_intensity: i16,
    color_enhancement: f32,
    glow_intensity: f32,
    sharpness: f32,
    exposure: f32,
    whites: f32,    // Make sure this parameter is being used
    blacks: f32,
    tint: &[TintAdjustment],
    apply_grayscale: bool,
) -> Result<(), image::ImageError> {
    let img = image::open(input_path)?.to_rgba8();
    
    // Apply adjustments in the correct order
    let mut processed = img.clone();
    
    // Apply exposure first
    processed = adjust_exposure(&processed, exposure);
    
    // Apply whites and blacks after exposure
    processed = adjust_whites(&processed, whites);
    processed = adjust_blacks(&processed, blacks);
    
    // Then apply other effects
    if apply_grayscale {
        processed = to_grayscale(&processed);
        img.clone();
    }
    
    processed = enhance_colors(&processed, color_enhancement);
    processed = sharpen(&processed, sharpness);
    processed = add_glow(&processed, glow_intensity);
    
    // Apply tint last
    for tint_adjustment in tint {
        processed = adjust_tint(&processed, tint_adjustment);
    }
    
    add_grain(&mut processed, grain_intensity);
    
    // Save the result
    processed.save(output_path)?;
    Ok(())
}


/// Adds a grain effect to the image by introducing random noise.
///
/// # Arguments
///
/// * `img` - A mutable reference to the image buffer.
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
/// Adjusts the whites of the image using a non-linear curve for more natural results.
///
/// # Arguments
///
/// * `img` - The input image buffer.
/// * `adjustment` - The whites adjustment factor. Positive values increase whites, negative values decrease whites.
///                 Recommended range: -1.0 to 1.0
///
/// # Returns
///
/// * An `ImageBuffer` with the whites adjusted.
pub fn adjust_whites(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    adjustment: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut adjusted_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Convert adjustment from 0-2 range to a more suitable range for processing
    let processed_adjustment = (adjustment - 1.0) * 128.0; // This maps 0-2 to -128 to +128

    for (x, y, pixel) in adjusted_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        for c in 0..3 {
            let value = original[c] as f32;
            
            // Apply non-linear adjustment to whites
            let adjusted = if processed_adjustment > 0.0 {
                // Increase whites: apply more adjustment to brighter pixels
                let factor = (value / 255.0).powf(0.5); // Non-linear factor
                value + (processed_adjustment * factor)
            } else {
                // Decrease whites: apply more adjustment to brighter pixels
                let factor = (value / 255.0).powf(2.0); // Non-linear factor
                value + (processed_adjustment * factor)
            };
            
            pixel[c] = adjusted.round().max(0.0).min(255.0) as u8;
        }
        pixel[3] = original[3]; // Preserve alpha channel
    }

    adjusted_img
}

/// Adjusts the blacks of the image using a non-linear curve for more natural results.
///
/// # Arguments
///
/// * `img` - The input image buffer.
/// * `adjustment` - The blacks adjustment factor. Positive values increase blacks, negative values decrease blacks.
///                 Recommended range: -1.0 to 1.0
///
/// # Returns
///
/// * An `ImageBuffer` with the blacks adjusted.
fn adjust_blacks(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    adjustment: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut adjusted_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Normalize adjustment to a reasonable range
    let adj = adjustment.max(-1.0).min(1.0);
    
    for (x, y, pixel) in adjusted_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        
        for c in 0..3 {
            let value = original[c] as f32 / 255.0; // Normalize to 0-1 range
            
            // Apply non-linear adjustment curve
            let adjusted = if adj > 0.0 {
                // For positive adjustment (increasing blacks)
                let threshold = 0.5 + (adj * 0.5); // Adjustable threshold
                if value < threshold {
                    let factor = (value / threshold).powf(1.0 + adj);
                    factor * threshold
                } else {
                    value
                }
            } else {
                // For negative adjustment (decreasing blacks)
                let threshold = 0.5 - (adj.abs() * 0.5);
                if value < threshold {
                    let factor = (value / threshold).powf(1.0 - adj.abs());
                    factor * threshold
                } else {
                    value
                }
            };
            
            // Convert back to u8 range
            pixel[c] = (adjusted * 255.0).round().max(0.0).min(255.0) as u8;
        }
        pixel[3] = original[3]; // Preserve alpha channel
    }
    
    adjusted_img
}

/// Represents a tint adjustment configuration
#[derive(Debug, Clone, Copy)]
pub struct TintAdjustment {
    pub hue: f32,         // Target hue (0-360)
    pub strength: f32,    // Tint strength (0.0 to 1.0)
    pub preserve_gray: f32, // How much to preserve gray values (0.0 to 1.0)
    pub luminance_mask: f32, // How much to respect original luminance (-1.0 to 1.0)
}

impl Default for TintAdjustment {
    fn default() -> Self {
        TintAdjustment {
            hue: 180.0,        // Default to cyan tint
            strength: 0.3,     // Moderate strength
            preserve_gray: 0.5, // Preserve some gray values
            luminance_mask: 0.0, // Neutral luminance masking
        }
    }
}

/// Converts RGB to HSL color space
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;

    if delta != 0.0 {
        s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        h = if r == max {
            (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
        } else if g == max {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        h *= 60.0;
    }

    (h, s, l)
}

/// Converts HSL to RGB color space
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let h = h / 360.0;

    let tr = (h + 1.0/3.0).rem_euclid(1.0);
    let tg = h;
    let tb = (h - 1.0/3.0).rem_euclid(1.0);

    let convert = |t: f32| -> f32 {
        if t < 1.0/6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0/2.0 {
            q
        } else if t < 2.0/3.0 {
            p + (q - p) * (2.0/3.0 - t) * 6.0
        } else {
            p
        }
    };

    (convert(tr), convert(tg), convert(tb))
}

/// Calculate the grayscale value of an RGB color
fn get_grayscale(r: f32, g: f32, b: f32) -> f32 {
    0.299 * r + 0.587 * g + 0.114 * b
}

/// Adjusts the tint of the image.
///
/// # Arguments
///
/// * `img` - The input image buffer.
/// * `tint` - The tint adjustment configuration.
///
/// # Returns
///
/// * An `ImageBuffer` with the tint adjusted.
pub fn adjust_tint(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    tint: &TintAdjustment,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut adjusted_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Create target tint color in HSL
    let target_rgb = hsl_to_rgb(tint.hue, 1.0, 0.5);
    let target_gray = get_grayscale(target_rgb.0, target_rgb.1, target_rgb.2);

    for (x, y, pixel) in adjusted_img.enumerate_pixels_mut() {
        let original = img.get_pixel(x, y);
        
        // Convert RGB to normalized float values
        let r = original[0] as f32 / 255.0;
        let g = original[1] as f32 / 255.0;
        let b = original[2] as f32 / 255.0;

        // Get original HSL and grayscale values
        let (orig_h, orig_s, orig_l) = rgb_to_hsl(r, g, b);
        let gray_value = get_grayscale(r, g, b);

        // Calculate gray preservation factor
        let gray_factor = if tint.preserve_gray > 0.0 {
            let color_difference = (r - gray_value).abs() +
                                 (g - gray_value).abs() +
                                 (b - gray_value).abs();
            (1.0 - (color_difference * 2.0)).max(0.0) * tint.preserve_gray
        } else {
            0.0
        };

        // Calculate luminance masking
        let luminance_factor = if tint.luminance_mask > 0.0 {
            if tint.luminance_mask > 0.0 {
                orig_l.powf(1.0 + tint.luminance_mask)
            } else {
                orig_l.powf(1.0 / (1.0 - tint.luminance_mask))
            }
        } else {
            1.0
        };

        // Blend original and tinted colors
        let tint_strength = tint.strength * (1.0 - gray_factor) * luminance_factor;
        
        // Create tinted color while preserving luminance
        let tinted = if tint_strength > 0.0 {
            let (_, new_s, _) = rgb_to_hsl(r, g, b);
            let new_h = tint.hue;
            let new_l = orig_l;
            
            // Blend between original and tinted color
            let tinted_rgb = hsl_to_rgb(new_h, new_s.min(1.0), new_l);
            (
                r + (tinted_rgb.0 - r) * tint_strength,
                g + (tinted_rgb.1 - g) * tint_strength,
                b + (tinted_rgb.2 - b) * tint_strength,
            )
        } else {
            (r, g, b)
        };

        // Set pixel values
        pixel[0] = (tinted.0 * 255.0).round().max(0.0).min(255.0) as u8;
        pixel[1] = (tinted.1 * 255.0).round().max(0.0).min(255.0) as u8;
        pixel[2] = (tinted.2 * 255.0).round().max(0.0).min(255.0) as u8;
        pixel[3] = original[3]; // Preserve alpha channel
    }

    adjusted_img
}


fn main() {
    let input_image_path = PathBuf::from("src/input.png");
    let output_image_path = PathBuf::from("src/output.png");

    // Check if the input file exists
    if !Path::new(&input_image_path).exists() {
        println!("Error: Input file '{}' not found.", input_image_path.display());
        println!("Please make sure the input image is in the same directory as the executable.");
        return;
    }


    let color_ranges = [TintAdjustment::default()];
    match apply_filter(&input_image_path, &output_image_path, 20, 0.5, 0.2, 0.8, 1.0, 1.0, 1.0, &color_ranges, true) {
        Ok(_) => println!("Image processing completed successfully."),
        Err(e) => println!("Error processing image: {}", e),
    }
}

