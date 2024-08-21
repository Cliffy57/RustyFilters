use log::{info, error};
use iced::{
    Alignment, Element, Length, Sandbox, Settings,
    widget::{Button, Column, Container, Image},
};
use iced::widget::image::Handle;
use native_dialog::FileDialog;
use std::path::PathBuf;
use std::fs;

use crate::image_processing;

pub struct ImageFilterApp {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    image_handle: Option<Handle>,
    filtered_image_handle: Option<Handle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectImage,
    ProcessImage,
}

impl Sandbox for ImageFilterApp {
    fn new() -> Self {
        ImageFilterApp {
            input_path: None,
            output_path: None,
            image_handle: None,
            filtered_image_handle: None,
        }
    }

    fn title(&self) -> String {
        String::from("RustyFilters")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage => {
                info!("Select Image button clicked");
                if let Ok(path) = FileDialog::new()
                    .add_filter("Image Files", &["png", "jpg", "jpeg"])
                    .show_open_single_file()
                {
                    if let Some(path) = path {
                        info!("File selected: {:?}", path);
                        self.input_path = Some(path.clone());
                        self.output_path = None;

                        // Load the image and create a handle
                        if let Ok(image_data) = fs::read(&path) {
                            self.image_handle = Some(Handle::from_memory(image_data));

                            // Apply the filter immediately for preview
                            let output_path = path.with_file_name("output_preview.png");
                            if image_processing::apply_filter(&path, &output_path).is_ok() {
                                if let Ok(filtered_image_data) = fs::read(&output_path) {
                                    self.filtered_image_handle = Some(Handle::from_memory(filtered_image_data));
                                } else {
                                    error!("Failed to read filtered image file");
                                }
                            } else {
                                error!("Error processing image");
                            }
                        } else {
                            error!("Failed to read image file");
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
                    if image_processing::apply_filter(input_path, &output_path).is_ok() {
                        self.output_path = Some(output_path);
                        info!("Image processed and saved");
                    } else {
                        error!("Error processing image");
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Button to select an image
        let select_button: Button<Message, iced::Theme, iced::Renderer> = Button::new("Select Image")
            .on_press(Message::SelectImage);

        // Button to apply the filter and save the output
        let apply_button: Button<Message, iced::Theme, iced::Renderer> = Button::new("Apply Filter")
            .on_press(Message::ProcessImage);

        // Column to hold the buttons and image previews
        let mut content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(select_button);

        // Display the original image preview if available
        if let Some(ref image_handle) = self.image_handle {
            let image_widget = Image::new(image_handle.clone())
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(300.0));
            content = content.push(image_widget);
        }

        // Display the filtered image preview if available
        if let Some(ref filtered_image_handle) = self.filtered_image_handle {
            let filtered_image_widget = Image::new(filtered_image_handle.clone())
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(300.0));
            content = content.push(filtered_image_widget);
            // Add the apply button only if there is a filtered image preview
            content = content.push(apply_button);
        }

        // Build and return the UI container
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
    
    type Message = Message;
}

use env_logger::Env;
pub fn run() -> iced::Result {
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    info!("Starting RustyFilters application");
    ImageFilterApp::run(Settings::default())
}
