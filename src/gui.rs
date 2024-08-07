use log::{info, error};
use iced::{
    Alignment, Element, Length, Sandbox, Settings,
    widget::Button,
};
use iced::widget::{Column, Container};

use native_dialog::FileDialog;
use std::path::PathBuf;

use crate::image_processing;

pub struct ImageFilterApp {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
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
                    } else {
                        info!("No file selected");
                    }
                } else {
                    error!("Error opening file dialog");
                }
            }
            Message::ProcessImage => {
                info!("Process Image button clicked");
                if let Some(input_path) = &self.input_path {
                    let output_path = input_path.with_file_name("output.png");
                    info!("Processing image: {:?}", input_path);
                    info!("Output path: {:?}", output_path);
                    if image_processing::apply_filter(input_path, &output_path).is_ok() {
                        info!("Image processing completed successfully");
                        self.output_path = Some(output_path);
                    } else {
                        error!("Error processing image");
                    }
                } else {
                    error!("No input image selected");
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let select_button: Button<Message, iced::Theme, iced::Renderer> = Button::new("Select Image")
            .on_press(Message::SelectImage);
        // Button to apply filters
        let process_button:  Button<Message, iced::Theme, iced::Renderer>  = Button::new("Apply Filters")
            .on_press(Message::ProcessImage);


        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(select_button);


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