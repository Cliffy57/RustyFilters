use std::path::PathBuf;
use iced::widget::image::Handle;
use crate::image_processing::TintAdjustment;

pub struct ImageFilterApp {
    pub input_path: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub image_handle: Option<Handle>,
    pub filtered_image_handle: Option<Handle>,
    pub grain_intensity: i16,
    pub color_enhancement: f32,
    pub glow_intensity: f32,
    pub sharpness: f32,
    pub exposure: f32,
    pub blacks: f32,
    pub whites: f32,
    pub tint: TintAdjustment,
    pub apply_grayscale: bool,
    pub(crate) show_initial_image: bool,
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    File,
    Edit,
    View,
    Help,
}

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
    ToggleImageView, // New message type
}