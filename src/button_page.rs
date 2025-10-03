// 🧊 iced_playground
// src/button_page.rs
use iced::widget::{button, column, container, text, vertical_space};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}

pub struct ButtonPage {
    button_pressed_count: u32,
}

impl Default for ButtonPage {
    fn default() -> Self {
        Self {
            button_pressed_count: 0,
        }
    }
}

impl ButtonPage {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.button_pressed_count += 1;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let code_block = r#"use iced::widget::button;

let button = button(text("Click me!"))
    .on_press(Message::ButtonPressed);"#;

        let button_content = column![
            text("Button Page").size(24),
            vertical_space(),
            text("This page demonstrates the button widget in Iced.").size(16),
            vertical_space(),
            button(text("Click me!"))
                .on_press(Message::ButtonPressed),
            vertical_space(),
            text(format!("Button pressed {} times", self.button_pressed_count)).size(16),
            vertical_space(),
            text("Code Example:").size(18),
            text(code_block).size(12),
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center);

        container(button_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}