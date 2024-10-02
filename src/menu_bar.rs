use iced::{Element, Row, Text, Button, Length, Command};
use iced::widget::button::State;

#[derive(Debug, Clone)]
pub enum MenuMessage {
  Menu1,
  Menu2,
  Menu3,
  Menu4,
  Menu5,
  Menu6,
  Menu7,
}

pub struct MenuBar {
  menu_states: [State; 7],
}

impl MenuBar {
  pub fn new() -> Self {
    MenuBar {
      menu_states: Default::default(),
    }
  }

  pub fn view(&mut self) -> Element<MenuMessage> {
    let menu_labels = [
      ("Menu 1", MenuMessage::Menu1),
      ("Menu 2", MenuMessage::Menu2),
      ("Menu 3", MenuMessage::Menu3),
      ("Menu 4", MenuMessage::Menu4),
      ("Menu 5", MenuMessage::Menu5),
      ("Menu 6", MenuMessage::Menu6),
      ("Menu 7", MenuMessage::Menu7),
    ];

    let mut row = Row::new().width(Length::Fill);

    for (i, (label, message)) in menu_labels.iter().enumerate() {
      row = row.push(Button::new(&mut self.menu_states[i], Text::new(label)).on_press(message.clone()));
    }

    row.into()
  }
}
