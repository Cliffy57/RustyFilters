use log::{info, error};
use iced::{
    Alignment, Element, Length, Sandbox, Settings,
    widget::{Button, Column, Container, Image, Text, Slider, Row, Space},
};
use iced::widget::image::Handle;
use native_dialog::FileDialog;
use std::path::PathBuf;
use std::fs;

use crate::image_processing::{self, TintAdjustment};
use std::process::Command;
use std::io;

fn optimize_image(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    // Create a temporary file path
    let temp_output_path = output_path.with_extension("temp.png");

    let status = Command::new("ffmpeg")
        .args(&[
            "-y", // Add this flag to force overwrite
            "-i", input_path.to_str().unwrap(),
            "-vf", "scale=iw*2:ih*2", // Example: scale up by 200%
            "-q:v", "1", // Set quality level (lower is better quality)
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
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    image_handle: Option<Handle>,
    filtered_image_handle: Option<Handle>,
    grain_intensity: i16,
    color_enhancement: f32,
    glow_intensity: f32,
    sharpness: f32,
    exposure: f32,
    blacks: f32,
    whites: f32,
    tint:TintAdjustment,
    apply_grayscale: bool, 
}

// Define the menu items
#[derive(Debug, Clone)]
pub enum MenuItem {
    File,
    Edit,
    View,
    Help,
}

// Define the messages that the application can handle
#[derive(Debug, Clone)]
pub enum Message {
    SelectImage,
    ProcessImage,
    GrainIntensityChanged(i16),
    ColorEnhancementChanged(f32),
    GlowIntensityChanged(f32),
    SharpnessChanged(f32),
    ExposureChanged(f32),
    WhitesChanged(f32),
    BlacksChanged(f32),
    TintChanged(TintAdjustment),
    ApplyGrayscale,
    MenuItemSelected(MenuItem),
}

// Implement the Sandbox trait for the application
impl Sandbox for ImageFilterApp {
    fn new() -> Self {
        ImageFilterApp {
            input_path: None,
            output_path: None,
            image_handle: None,
            filtered_image_handle: None,
            grain_intensity: 10,
            color_enhancement: 1.05,
            glow_intensity: 0.05,
            sharpness: 0.8,
            exposure: 1.0,
            blacks:1.0,
            whites:1.0,
            tint: TintAdjustment::default(),
            apply_grayscale: false, 
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
                                self.update_preview();
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
                if let Some(ref input_path) = self.input_path {
                    let output_path = input_path.with_file_name("output.png");
                    if image_processing::apply_filter(
                        input_path,
                        &output_path,
                        self.grain_intensity,
                        self.color_enhancement,
                        self.glow_intensity,
                        self.sharpness,
                        self.exposure,
                        self.whites,
                        self.blacks,
                        &[self.tint],
                        self.apply_grayscale
                    ).is_ok() {
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
                self.update_preview();
            }
            Message::ColorEnhancementChanged(enhancement) => {
                self.color_enhancement = enhancement;
                self.update_preview();
            }
            Message::GlowIntensityChanged(intensity) => {
                self.glow_intensity = intensity;
                self.update_preview();
            }
            Message::SharpnessChanged(sharpness) => {
                self.sharpness = sharpness;
                self.update_preview();
            }
            Message::ExposureChanged(exposure) => {
                self.exposure = exposure;
                self.update_preview();
            }
            Message::WhitesChanged(whites) => {
                self.whites = whites;
                self.update_preview();
            }
            Message::BlacksChanged(blacks) => {
                self.blacks = blacks;
                self.update_preview();
            }
            Message::TintChanged(tint) => {
                self.tint = tint;
                self.update_preview();
            }
            Message::ApplyGrayscale => {
                self.apply_grayscale = !self.apply_grayscale;
                self.update_preview();
            }
            Message::MenuItemSelected(menu_item) => {
                info!("Menu item selected: {:?}", menu_item);
                // Handle menu item selection
                match menu_item {
                    MenuItem::File => {
                        // Handle File menu actions
                    }
                    MenuItem::Edit => {
                        // Handle Edit menu actions
                    }
                    MenuItem::View => {
                        // Handle View menu actions
                    }
                    MenuItem::Help => {
                        // Handle Help menu actions
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let select_button = Button::new("Select Image")
            .on_press(Message::SelectImage);

        let apply_button = Button::new("Apply Filter")
            .on_press(Message::ProcessImage);

        let grain_slider = Slider::new(0..=20, self.grain_intensity, Message::GrainIntensityChanged)
            .step(1i16);

        let color_enhancement_slider = Slider::new(1.0..=1.2, self.color_enhancement, |v| Message::ColorEnhancementChanged(v))
            .step(0.01);

        let glow_intensity_slider = Slider::new(0.0..=0.2, self.glow_intensity, |v| Message::GlowIntensityChanged(v))
            .step(0.01);

        let sharpness_slider = Slider::new(0.0..=2.0, self.sharpness, |v| Message::SharpnessChanged(v))
            .step(0.1);
        let exposure_slider = Slider::new(0.0..=2.0, self.exposure, |v| Message::ExposureChanged(v))
            .step(0.1);
        let blacks_slider = Slider::new(0.0..=2.0, self.blacks, |v| Message::BlacksChanged(v))
        .step(0.1);
        let whites_slider = Slider::new(0.0..=2.0, self.whites, |v| Message::WhitesChanged(v))
        .step(0.1);
        let tint_slider = Slider::new(0.0..=360.0, self.tint.hue, |v| Message::TintChanged(TintAdjustment { hue: v, strength: self.tint.strength, preserve_gray: self.tint.preserve_gray, luminance_mask: self.tint.luminance_mask }))
            .step(1.0);
        let grayscale_button_label = if self.apply_grayscale {
            "Remove Grayscale"
        } else {
            "Apply Grayscale"
        };

        let grayscale_button = Button::new(grayscale_button_label)
            .on_press(Message::ApplyGrayscale);

        let side_panel = Container::new(
            Column::new()
                .spacing(10)
                .padding(20)
                .push(Text::new("Controls").size(20))
                .push(Container::new(Text::new(format!("Grain Intensity: {}", self.grain_intensity)))
                    .padding(5))
                .push(grain_slider)
                .push(Container::new(Text::new(format!("Color Enhancement: {:.2}", self.color_enhancement)))
                    .padding(5))
                .push(color_enhancement_slider)
                .push(Container::new(Text::new(format!("Glow Intensity: {:.2}", self.glow_intensity)))
                    .padding(5))
                .push(glow_intensity_slider)
                .push(Container::new(Text::new(format!("Sharpness: {:.1}", self.sharpness)))
                    .padding(5))
                .push(sharpness_slider)
                .push(Container::new(Text::new(format!("Exposure: {:.1}", self.exposure)))
                    .padding(5))
                .push(exposure_slider)
                .push(Container::new(Text::new(format!("Blacks: {:.1}", self.blacks)))
                    .padding(5))
                .push(blacks_slider)
                .push(Container::new(Text::new(format!("Whites: {:.1}", self.whites)))
                    .padding(5))
                .push(whites_slider)
                .push(Container::new(Text::new(format!("Tint: {:?}", self.tint)))
                    .padding(5))
                .push(tint_slider)
                .push(select_button)
                .push(grayscale_button) // Add the grayscale button to the side panel
        )
        .width(Length::Fixed(250.0))
        .padding(10)
        .center_x();

        // Main content
        let mut main_content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("Image Preview").size(20));

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

        // Create the menu bar
        let menu_bar = self.create_menu_bar();

        // Combine menu bar, side panel, and main content in a column
        let content = Column::new()
            .spacing(20)
            .push(menu_bar)
            .push(Row::new()
                .spacing(20)
                .push(side_panel)
                .push(Container::new(main_content).padding(20).center_x().center_y()));

        // Build and return the UI container
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
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

impl ImageFilterApp {
    fn create_menu_bar(&self) -> Row<Message> {
        let file_menu = Button::new("File")
            .on_press(Message::MenuItemSelected(MenuItem::File));
        
        let edit_menu = Button::new("Edit")
            .on_press(Message::MenuItemSelected(MenuItem::Edit));
        
        let view_menu = Button::new("View")
            .on_press(Message::MenuItemSelected(MenuItem::View));
        
        let help_menu = Button::new("Help")
            .on_press(Message::MenuItemSelected(MenuItem::Help));
        
        Row::new()
            .spacing(20)
            .push(file_menu)
            .push(edit_menu)
            .push(view_menu)
            .push(help_menu)
    }

    fn update_preview(&mut self) {
        if let Some(ref input_path) = self.input_path {
            let output_path = input_path.with_file_name("output_preview.png");
            if image_processing::apply_filter(
                input_path,
                &output_path,
                self.grain_intensity,
                self.color_enhancement,
                self.glow_intensity,
                self.sharpness,
                self.exposure,
                self.whites,
                self.blacks,
                &[self.tint],
                self.apply_grayscale
            ).is_ok() {
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