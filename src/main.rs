mod app;
mod commands;
mod image_processing;
mod ui;

use env_logger::Env;
use iced::{Application, Settings};
use app::ImageFilterApp;
use std::io::Cursor;

fn main() -> iced::Result {
    // Load icon from file
    let icon_data = include_bytes!("../RustyFilters.png");
    let icon = image::load(Cursor::new(icon_data), image::ImageFormat::Png)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = icon.dimensions();
    let icon = iced::window::icon::from_rgba(icon.into_raw(), width, height)
        .expect("Failed to create icon");

    // Create settings with icon
    let mut settings = Settings::default();
    settings.window.icon = Some(icon);

    // Initialize logger and run app
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    log::info!("Starting RustyFilters application");
    ImageFilterApp::run(settings)
}