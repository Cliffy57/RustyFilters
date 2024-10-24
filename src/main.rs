mod app;
mod commands;
mod image_processing;
mod ui;

use env_logger::Env;
use iced::{Application, Settings};
use app::ImageFilterApp;

fn main() -> iced::Result {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    log::info!("Starting RustyFilters application");
    ImageFilterApp::run(Settings::default())
}