use log::info;
use env_logger::Env;
use iced::{Application, Settings};

mod gui;
mod image_processing;

fn main() -> iced::Result {
    // Initialize the logger
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    info!("Starting RustyFilters application");

    // Run the application
    gui::ImageFilterApp::run(Settings::default())
}