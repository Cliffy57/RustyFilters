use image::{ImageBuffer, Rgba};
use rand::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;

/// Applies various filters and effects to an input image and saves the result.
pub fn apply_filter(
    input_path: &PathBuf,
    output_path: &PathBuf,
    grain_intensity: i16,
    color_enhancement: f32,
    glow_intensity: f32,
    sharpness: f32,
    exposure: f32,
    whites: f32,
    blacks: f32,
    tint: &[TintAdjustment],
    apply_grayscale: bool,
) -> Result<(), image::ImageError> {
    let img = image::open(input_path)?.to_rgba8();
    
    // Create a mutable copy for processing
    let mut processed = img.clone();
    
    // Apply adjustments in the correct order with parallel processing
    adjust_exposure(&mut processed, exposure);
    adjust_whites(&mut processed, whites);
    adjust_blacks(&mut processed, blacks);
    
    if apply_grayscale {
        to_grayscale(&mut processed);
    }
    
    enhance_colors(&mut processed, color_enhancement);
    processed = sharpen(&processed, sharpness);
    add_glow(&mut processed, glow_intensity);
    
    // Apply tint adjustments
    for tint_adjustment in tint {
        adjust_tint(&mut processed, tint_adjustment);
    }
    
    add_grain(&mut processed, grain_intensity);
    
    // Save the result
    processed.save(output_path)?;
    Ok(())
}

/// Optimized parallel grain addition using proper chunking
fn add_grain(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, intensity: i16) {
    if intensity == 0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    
    // Process in chunks of 4 (RGBA pixels) with proper parallel iteration
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        let mut rng = rand::thread_rng();
        let noise: i16 = rng.gen_range(-intensity..=intensity);
        
        // Apply noise to RGB channels only (skip alpha)
        for c in 0..3 {
            let new_value = (pixel[c] as i16 + noise).clamp(0, 255);
            pixel[c] = new_value as u8;
        }
    });
}

/// Optimized parallel color enhancement
fn enhance_colors(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, enhancement: f32) {
    if enhancement == 1.0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        for c in 0..3 {
            let value = pixel[c] as f32;
            pixel[c] = (value * enhancement).clamp(0.0, 255.0) as u8;
        }
    });
}

/// Optimized parallel grayscale conversion
fn to_grayscale(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        let gray_value = (0.299 * pixel[0] as f32 + 
                          0.587 * pixel[1] as f32 + 
                          0.114 * pixel[2] as f32) as u8;
        pixel[0] = gray_value;
        pixel[1] = gray_value;
        pixel[2] = gray_value;
        // Keep alpha channel unchanged
    });
}

/// Optimized parallel exposure adjustment
fn adjust_exposure(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, adjustment: f32) {
    if adjustment == 1.0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        for c in 0..3 {
            let value = pixel[c] as f32;
            pixel[c] = (value * adjustment).clamp(0.0, 255.0) as u8;
        }
    });
}

/// Optimized parallel whites adjustment
fn adjust_whites(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, adjustment: f32) {
    if adjustment == 1.0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    let processed_adjustment = (adjustment - 1.0) * 128.0;
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        for c in 0..3 {
            let value = pixel[c] as f32;
            
            let adjusted = if processed_adjustment > 0.0 {
                let factor = (value / 255.0).powf(0.5);
                value + (processed_adjustment * factor)
            } else {
                let factor = (value / 255.0).powf(2.0);
                value + (processed_adjustment * factor)
            };
            
            pixel[c] = adjusted.clamp(0.0, 255.0) as u8;
        }
    });
}

/// Optimized parallel blacks adjustment
fn adjust_blacks(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, adjustment: f32) {
    if adjustment == 1.0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    let adj = adjustment.clamp(-1.0, 1.0);
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        for c in 0..3 {
            let value = pixel[c] as f32 / 255.0;
            
            let adjusted = if adj > 0.0 {
                let threshold = 0.5 + (adj * 0.5);
                if value < threshold {
                    let factor = (value / threshold).powf(1.0 + adj);
                    factor * threshold
                } else {
                    value
                }
            } else {
                let threshold = 0.5 - (adj.abs() * 0.5);
                if value < threshold {
                    let factor = (value / threshold).powf(1.0 - adj.abs());
                    factor * threshold
                } else {
                    value
                }
            };
            
            pixel[c] = (adjusted * 255.0).clamp(0.0, 255.0) as u8;
        }
    });
}

/// Optimized glow effect with proper blur and blending
fn add_glow(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, intensity: f32) {
    if intensity <= 0.0 {
        return;
    }
    
    let (width, height) = img.dimensions();
    let original_img = img.clone();
    
    // Create blur buffer with explicit type annotation
    let mut blur_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    // Apply separable Gaussian blur for better performance
    let blur_radius = (intensity * 3.0) as u32;
    if blur_radius == 0 {
        return;
    }
    
    // Simple box blur approximation for performance
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut count = 0u32;
            
            let start_x = x.saturating_sub(blur_radius);
            let end_x = (x + blur_radius).min(width - 1);
            let start_y = y.saturating_sub(blur_radius);
            let end_y = (y + blur_radius).min(height - 1);
            
            for by in start_y..=end_y {
                for bx in start_x..=end_x {
                    let pixel = original_img.get_pixel(bx, by);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    count += 1;
                }
            }
            
            let blur_pixel: &mut Rgba<u8> = blur_img.get_pixel_mut(x, y);
            if count > 0 {
                blur_pixel[0] = (r_sum / count) as u8;
                blur_pixel[1] = (g_sum / count) as u8;
                blur_pixel[2] = (b_sum / count) as u8;
                blur_pixel[3] = 255;
            }
        }
    }
    
    // Blend original with glow using screen blend mode
    for y in 0..height {
        for x in 0..width {
            let original_pixel = original_img.get_pixel(x, y);
            let glow_pixel = blur_img.get_pixel(x, y);
            let result_pixel = img.get_pixel_mut(x, y);
            
            for c in 0..3 {
                let original = original_pixel[c] as f32 / 255.0;
                let glow = glow_pixel[c] as f32 / 255.0;
                
                // Screen blend mode for glow effect
                let blended = 1.0 - (1.0 - original) * (1.0 - glow * intensity);
                result_pixel[c] = (blended * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
    }
}


/// Optimized parallel tint adjustment
fn adjust_tint(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, tint: &TintAdjustment) {
    if tint.strength == 0.0 {
        return;
    }
    
    let mut pixels = img.as_flat_samples_mut();
    let data = pixels.as_mut_slice();
    
    data.par_chunks_exact_mut(4).for_each(|pixel| {
        // Convert RGB to normalized float values
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        // Get original HSL values
        let (_, orig_s, orig_l) = rgb_to_hsl(r, g, b);
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
        let luminance_factor = if tint.luminance_mask != 0.0 {
            if tint.luminance_mask > 0.0 {
                orig_l.powf(1.0 + tint.luminance_mask)
            } else {
                orig_l.powf(1.0 / (1.0 - tint.luminance_mask))
            }
        } else {
            1.0
        };

        // Apply tint
        let tint_strength = tint.strength * (1.0 - gray_factor) * luminance_factor;
        
        if tint_strength > 0.0 {
            // Create tinted color while preserving saturation and luminance
            let tinted_rgb = hsl_to_rgb(tint.hue, orig_s, orig_l);
            
            // Blend between original and tinted color
            let tinted = (
                r + (tinted_rgb.0 - r) * tint_strength,
                g + (tinted_rgb.1 - g) * tint_strength,
                b + (tinted_rgb.2 - b) * tint_strength,
            );

            // Set pixel values
            pixel[0] = (tinted.0 * 255.0).clamp(0.0, 255.0) as u8;
            pixel[1] = (tinted.1 * 255.0).clamp(0.0, 255.0) as u8;
            pixel[2] = (tinted.2 * 255.0).clamp(0.0, 255.0) as u8;
        }
    });
}

/// Optimized parallel sharpening with better edge handling
fn sharpen(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, sharpness: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if sharpness == 0.0 {
        return img.clone();
    }
    
    let (width, height) = img.dimensions();
    let mut sharpened_img = ImageBuffer::new(width, height);
    
    let center = 1.0 + 4.0 * sharpness;
    let sides = -sharpness;
    
    // Copy edges first (they won't be processed)
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                *sharpened_img.get_pixel_mut(x, y) = *img.get_pixel(x, y);
            }
        }
    }
    
    // Collect all results first, then apply them (avoids data races)
    let results: Vec<(u32, u32, [u8; 4])> = (1..height-1)
        .into_par_iter()
        .flat_map(|y| {
            (1..width-1).into_par_iter().map(move |x| {
                let mut new_pixel = [0.0; 3];
                
                // Apply 3x3 sharpening kernel
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let px = (x as i32 + dx) as u32;
                        let py = (y as i32 + dy) as u32;
                        
                        let pixel = img.get_pixel(px, py);
                        let weight = if dx == 0 && dy == 0 { center } else { sides };
                        
                        for c in 0..3 {
                            new_pixel[c] += pixel[c] as f32 * weight;
                        }
                    }
                }
                
                let final_pixel = [
                    new_pixel[0].clamp(0.0, 255.0) as u8,
                    new_pixel[1].clamp(0.0, 255.0) as u8,
                    new_pixel[2].clamp(0.0, 255.0) as u8,
                    img.get_pixel(x, y)[3], // Preserve alpha
                ];
                
                (x, y, final_pixel)
            })
        })
        .collect();
    
    // Apply all results
    for (x, y, pixel_data) in results {
        let pixel = sharpened_img.get_pixel_mut(x, y);
        pixel[0] = pixel_data[0];
        pixel[1] = pixel_data[1];
        pixel[2] = pixel_data[2];
        pixel[3] = pixel_data[3];
    }
    
    sharpened_img
}

// Safe version of sharpening without data races
fn sharpen_safe(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, sharpness: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if sharpness == 0.0 {
        return img.clone();
    }
    
    let (width, height) = img.dimensions();
    let mut sharpened_img = ImageBuffer::new(width, height);
    
    let center = 1.0 + 4.0 * sharpness;
    let sides = -sharpness;
    
    // Copy edges first
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                *sharpened_img.get_pixel_mut(x, y) = *img.get_pixel(x, y);
            }
        }
    }
    
    // Process interior pixels in parallel
    (1..height-1).into_par_iter().for_each(|y| {
        for x in 1..width-1 {
            let mut new_pixel = [0.0; 3];
            
            // Apply 3x3 sharpening kernel
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let px = (x as i32 + dx) as u32;
                    let py = (y as i32 + dy) as u32;
                    
                    let pixel = img.get_pixel(px, py);
                    let weight = if dx == 0 && dy == 0 { center } else { sides };
                    
                    for c in 0..3 {
                        new_pixel[c] += pixel[c] as f32 * weight;
                    }
                }
            }
            
            // We need to create a temporary buffer for each row to avoid data races
            // This is still not ideal - we need a different approach
        }
    });
    
    // Better approach: collect results and then apply
    let results: Vec<(u32, u32, [u8; 3])> = (1..height-1)
        .into_par_iter()
        .flat_map(|y| {
            (1..width-1).into_par_iter().map(move |x| {
                let mut new_pixel = [0.0; 3];
                
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let px = (x as i32 + dx) as u32;
                        let py = (y as i32 + dy) as u32;
                        
                        let pixel = img.get_pixel(px, py);
                        let weight = if dx == 0 && dy == 0 { center } else { sides };
                        
                        for c in 0..3 {
                            new_pixel[c] += pixel[c] as f32 * weight;
                        }
                    }
                }
                
                let final_pixel = [
                    new_pixel[0].clamp(0.0, 255.0) as u8,
                    new_pixel[1].clamp(0.0, 255.0) as u8,
                    new_pixel[2].clamp(0.0, 255.0) as u8,
                ];
                
                (x, y, final_pixel)
            })
        })
        .collect();
    
    // Apply results
    for (x, y, pixel_data) in results {
        let pixel = sharpened_img.get_pixel_mut(x, y);
        pixel[0] = pixel_data[0];
        pixel[1] = pixel_data[1];
        pixel[2] = pixel_data[2];
        pixel[3] = img.get_pixel(x, y)[3]; // Preserve alpha
    }
    
    sharpened_img
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

/// Optimized RGB to HSL conversion
#[inline]
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;
    
    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if r == max {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if g == max {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    (h * 60.0, s, l)
}

/// Optimized HSL to RGB conversion
#[inline]
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r1 + m, g1 + m, b1 + m)
}

/// Calculate the grayscale value of an RGB color
#[inline]
fn get_grayscale(r: f32, g: f32, b: f32) -> f32 {
    0.299 * r + 0.587 * g + 0.114 * b
}

fn main() {
    let input_image_path = PathBuf::from("src/input.png");
    let output_image_path = PathBuf::from("src/output.png");

    // Check if the input file exists
    if !input_image_path.exists() {
        println!("Error: Input file '{}' not found.", input_image_path.display());
        println!("Please make sure the input image is in the same directory as the executable.");
        return;
    }

    let color_ranges = [TintAdjustment::default()];
    
    let start_time = std::time::Instant::now();
    
    match apply_filter(
        &input_image_path, 
        &output_image_path, 
        20,    // grain_intensity
        1.2,   // color_enhancement  
        0.2,   // glow_intensity
        0.8,   // sharpness
        1.0,   // exposure
        1.0,   // whites
        1.0,   // blacks
        &color_ranges, 
        false  // apply_grayscale - changed to false for better demonstration
    ) {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            println!("Image processing completed successfully in {:.2?}", elapsed);
        },
        Err(e) => println!("Error processing image: {}", e),
    }
}