use crate::app::{ImageFilterApp, Message};
use crate::image_processing;
use iced::widget::image::Handle;
use native_dialog::FileDialog;
use std::fs;
use std::path::PathBuf;
use log::{info, error};

pub fn handle_message(app: &mut ImageFilterApp, message: Message) {
    match message {
        Message::SelectImage => select_image(app),
        Message::ProcessImage => process_image(app),
        Message::GrainIntensityChanged(intensity) => {
            app.grain_intensity = intensity;
            app.update_preview();
        }
        Message::ColorEnhancementChanged(enhancement) => {
            app.color_enhancement = enhancement;
            app.update_preview();
        }
        Message::GlowIntensityChanged(intensity) => {
            app.glow_intensity = intensity;
            app.update_preview();
        }
        Message::SharpnessChanged(sharpness) => {
            app.sharpness = sharpness;
            app.update_preview();
        }
        Message::ExposureChanged(exposure) => {
            app.exposure = exposure;
            app.update_preview();
        }
        Message::WhitesChanged(whites) => {
            app.whites = whites;
            app.update_preview();
        }
        Message::BlacksChanged(blacks) => {
            app.blacks = blacks;
            app.update_preview();
        }
        Message::TintChanged(tint) => {
            app.tint = tint;
            app.update_preview();
        }
        Message::ApplyGrayscale => {
            app.apply_grayscale = !app.apply_grayscale;
            app.update_preview();
        }
        Message::MenuItemSelected(menu_item) => {
            info!("Menu item selected: {:?}", menu_item);
            // Handle menu item selection
        }
    }
}

fn select_image(app: &mut ImageFilterApp) {
    info!("Select Image button clicked");
    if let Ok(path) = FileDialog::new()
        .add_filter("Image Files", &["png", "jpg", "jpeg"])
        .show_open_single_file()
    {
        if let Some(path) = path {
            info!("File selected: {:?}", path);
            app.input_path = Some(path.clone());
            app.output_path = None;

            match fs::read(&path) {
                Ok(image_data) => {
                    app.image_handle = Some(Handle::from_memory(image_data));
                    app.update_preview();
                }
                Err(e) => {
                    error!("Failed to read image file: {:?}", e);
                }
            }
        } else {
            info!("No file selected");
        }
    } else {
        error!("Error opening file dialog");
    }
}

fn process_image(app: &mut ImageFilterApp) {
    if let Some(ref input_path) = app.input_path {
        let output_path = input_path.with_file_name("output.png");
        if image_processing::apply_filter(
            input_path,
            &output_path,
            app.grain_intensity,
            app.color_enhancement,
            app.glow_intensity,
            app.sharpness,
            app.exposure,
            app.whites,
            app.blacks,
            &[app.tint],
            app.apply_grayscale
        ).is_ok() {
            if let Err(e) = optimize_image(&output_path, &output_path) {
                error!("Failed to optimize image: {:?}", e);
            } else {
                app.output_path = Some(output_path);
                info!("Image processed, optimized, and saved");
            }
        } else {
            error!("Error processing image");
        }
    }
}

fn optimize_image(input_path: &PathBuf, output_path: &PathBuf) -> std::io::Result<()> {
    let temp_output_path = output_path.with_extension("temp.png");

    let status = std::process::Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i", input_path.to_str().unwrap(),
            "-vf", "scale=iw*2:ih*2",
            "-q:v", "1",
            temp_output_path.to_str().unwrap(),
        ])
        .status()?;
    
    info!("ffmpeg command status: {}", status);
    
    if status.success() {
        fs::rename(temp_output_path, output_path)?;
        Ok(())
    } else {
        let _ = fs::remove_file(temp_output_path);
        Err(std::io::Error::new(std::io::ErrorKind::Other, "ffmpeg command failed"))
    }
}