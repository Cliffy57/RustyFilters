use log::{info, error}; // Import logging macros
use iced::{
    Alignment, Element, Length, Sandbox, Settings,
    widget::{Button, Column, Container, Image, Text, Slider, Row},
};
use iced::widget::image::Handle;
use native_dialog::FileDialog;
use std::path::PathBuf;
use std::fs; // Import fs module for file operations

use crate::image_processing; // Import custom image processing module
use std::process::Command;
use std::io;

fn optimize_image(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    // Create a temporary file path
    let temp_output_path = output_path.with_extension("temp.png");

    let status = Command::new("ffmpeg")
        .args(&[
            "-y", // Add this flag to force overwrite
            "-i", input_path.to_str().unwrap(),
            "-vf", "scale=iw*0.5:ih*0.5", // Example: scale down by 50%
            "-q:v", "2", // Set quality level (lower is better quality)
            temp_output_path.to_str().unwrap(),
        ])
        .status()?;
    
    // Info about the ffmpeg command status
    info!("ffmpeg command status: {}", status);
    
    if status.success() {
        // Replace the original file with the temporary file
        fs::rename(temp_output_path, output_path)?;
        Ok(())
    } else {
        // Clean up the temporary file if the command failed
        let _ = fs::remove_file(temp_output_path);
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg command failed"))
    }
}

// Define the main application structure
pub struct ImageFilterApp {
    input_path: Option<PathBuf>, // Path to the input image
    output_path: Option<PathBuf>, // Path to the output image
    image_handle: Option<Handle>, // Handle for the original image
    filtered_image_handle: Option<Handle>, // Handle for the filtered image
    grain_intensity: i16, // Grain intensity value
}

// Define the messages that the application can handle
#[derive(Debug, Clone)]
pub enum Message {
    SelectImage, // Message for selecting an image
    ProcessImage, // Message for processing the image
    GrainIntensityChanged(i16), // Message for changing grain intensity
}

// Implement the Sandbox trait for the application
impl Sandbox for ImageFilterApp {
    // Initialize the application state
    fn new() -> Self {
        ImageFilterApp {
            input_path: None,
            output_path: None,
            image_handle: None,
            filtered_image_handle: None,
            grain_intensity: 10, // Default grain intensity
        }
    }

    // Set the application title
    fn title(&self) -> String {
        String::from("RustyFilters")
    }

    // Handle messages and update the application state
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage => {
                info!("Select Image button clicked");
                // Open file dialog to select an image
                if let Ok(path) = FileDialog::new()
                    .add_filter("Image Files", &["png", "jpg", "jpeg"])
                    .show_open_single_file()
                {
                    if let Some(path) = path {
                        info!("File selected: {:?}", path);
                        self.input_path = Some(path.clone());
                        self.output_path = None;

                        // Load the image and create a handle
                        match fs::read(&path) {
                            Ok(image_data) => {
                                self.image_handle = Some(Handle::from_memory(image_data));

                                // Apply the filter immediately for preview
                                let output_path = path.with_file_name("output_preview.png");
                                if image_processing::apply_filter(&path, &output_path, self.grain_intensity).is_ok() {
                                    match fs::read(&output_path) {
                                        Ok(filtered_image_data) => {
                                            self.filtered_image_handle = Some(Handle::from_memory(filtered_image_data));
                                        }
                                        Err(e) => {
                                            error!("Failed to read filtered image file: {:?}", e);
                                        }
                                    }
                                } else {
                                    error!("Error processing image");
                                }
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
            Message::ProcessImage => {
                // Apply the filter and save the output image
                if let Some(ref input_path) = self.input_path {
                    let output_path = input_path.with_file_name("output.png");
                    if image_processing::apply_filter(input_path, &output_path, self.grain_intensity).is_ok() {
                        // Optimize the output image using ffmpeg
                        if let Err(e) = optimize_image(&output_path, &output_path) {
                            error!("Failed to optimize image: {:?}", e);
                        } else {
                            self.output_path = Some(output_path);
                            info!("Image processed, optimized, and saved");
                        }
                    } else {
                        error!("Error processing image");
                    }
                }
            }
            Message::GrainIntensityChanged(intensity) => {
                self.grain_intensity = intensity;
                info!("Grain intensity changed to {}", intensity);
                // Reapply the filter for preview
                if let Some(ref input_path) = self.input_path {
                    let output_path = input_path.with_file_name("output_preview.png");
                    if image_processing::apply_filter(input_path, &output_path, self.grain_intensity).is_ok() {
                        match fs::read(&output_path) {
                            Ok(filtered_image_data) => {
                                self.filtered_image_handle = Some(Handle::from_memory(filtered_image_data));
                            }
                            Err(e) => {
                                error!("Failed to read filtered image file: {:?}", e);
                            }
                        }
                    } else {
                        error!("Error processing image");
                    }
                }
            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        // Button to select an image
        let select_button = Button::new("Select Image")
            .on_press(Message::SelectImage);

        // Button to apply the filter and save the output
        let apply_button = Button::new("Apply Filter")
            .on_press(Message::ProcessImage);

        // Slider to control grain intensity
        let grain_slider = Slider::new(0..=20, self.grain_intensity, Message::GrainIntensityChanged);

        // Side panel content
        let side_panel = Column::new()
            .spacing(20)
            .padding(20)
            .width(Length::Fixed(200.0))
            .push(Text::new("Controls"))
            .push(Text::new(format!("Grain Intensity: {}", self.grain_intensity)))
            .push(grain_slider)
            .push(select_button);

        // Main content
        let mut main_content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center);

        // Display the original image preview if available
        if let Some(ref image_handle) = self.image_handle {
            let image_widget = Image::new(image_handle.clone())
                .width(Length::Fill)
                .height(Length::Fill);
            main_content = main_content.push(image_widget);
        }

        // Display the filtered image preview if available
        if let Some(ref filtered_image_handle) = self.filtered_image_handle {
            let filtered_image_widget = Image::new(filtered_image_handle.clone())
                .width(Length::Fill)
                .height(Length::Fill);
            main_content = main_content.push(filtered_image_widget);
            // Add the apply button only if there is a filtered image preview
            main_content = main_content.push(apply_button);
        }

        // Combine side panel and main content in a row
        let content = Row::new()
            .push(side_panel)
            .push(main_content);

        // Build and return the UI container
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // Set the application theme
    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    // Set the application style
    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    // Set the scale factor for the application
    fn scale_factor(&self) -> f64 {
        1.0
    }

    // Run the application
    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
    
    type Message = Message;
}

// Implement the Drop trait for the application
impl Drop for ImageFilterApp {
    fn drop(&mut self) {
        if let Some(ref input_path) = self.input_path {
            let preview_path = input_path.with_file_name("output_preview.png");
            if preview_path.exists() {
                match fs::remove_file(&preview_path) {
                    Ok(_) => info!("Preview file deleted successfully"),
                    Err(e) => error!("Failed to delete preview file: {:?}", e),
                }
            }
        }
    }
}

// Initialize the logger and run the application
use env_logger::Env;
pub fn run() -> iced::Result {
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    info!("Starting RustyFilters application");
    ImageFilterApp::run(Settings::default())
}