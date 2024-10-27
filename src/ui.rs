use std::fs;

use iced::{
  widget::{image::Handle, Button, Column, Container, Image, Row, Slider, Text}, Alignment, Element, Length, Sandbox, Settings
};
use log::{error, info};
use crate::{app::{ImageFilterApp, MenuItem, Message}, image_processing::{self, TintAdjustment}};
use crate::commands::handle_message;

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
            blacks: 1.0,
            whites: 1.0,
            tint: TintAdjustment::default(),
            apply_grayscale: false,
            show_initial_image: false,
        }
    }

    fn title(&self) -> String {
        String::from("RustyFilters")
    }

    fn update(&mut self, message: Message) {
        handle_message(self, message);
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

        let toggle_image_button_label = if self.show_initial_image {
            "Show Filtered Image"
        } else {
            "Show Initial Image"
        };

        let toggle_image_button = Button::new(toggle_image_button_label)
            .on_press(Message::ToggleImageView);

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
                .push(grayscale_button)
                .push(toggle_image_button) // Add the toggle image button
        )
        .width(Length::Fixed(250.0))
        .padding(10)
        .center_x();

        let mut main_content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("Image Preview").size(20));

        if self.show_initial_image {
            if let Some(ref image_handle) = self.image_handle {
                let image_widget = Image::new(image_handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill);
                main_content = main_content.push(image_widget);
            }
        } else {
            if let Some(ref filtered_image_handle) = self.filtered_image_handle {
                let filtered_image_widget = Image::new(filtered_image_handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill);
                main_content = main_content.push(filtered_image_widget);
                main_content = main_content.push(apply_button);
            }
        }

        let menu_bar = self.create_menu_bar();

        let content = Column::new()
            .spacing(20)
            .push(menu_bar)
            .push(Row::new()
                .spacing(20)
                .push(side_panel)
                .push(Container::new(main_content).padding(20).center_x().center_y()));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
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

  pub fn update_preview(&mut self) {
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