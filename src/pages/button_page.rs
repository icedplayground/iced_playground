// 🧊 iced_playground
// src/pages/button_page.rs
use iced::widget::{button, column, container, row, text};
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
        // buttons
        let buttons = row![
            button(text("PRIMARY"))
                .style(button::primary)
                .on_press(Message::ButtonPressed),
            button(text("SECONDARY"))
                .style(button::secondary)
                .on_press(Message::ButtonPressed),
            button(text("DANGER"))
                .style(button::danger)
                .on_press(Message::ButtonPressed),
            button(text("SUCCESS"))
                .style(button::success)
                .on_press(Message::ButtonPressed),
            button(text("TEXT"))
                .style(button::text)
                .on_press(Message::ButtonPressed),
        ]
        .spacing(10);

        // button_content
        let button_content = column![
            text("BUTTON").size(24),
            buttons,
            text(format!(
                "BUTTON PRESSED {} TIMES",
                self.button_pressed_count
            ))
            .size(16),
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



// ==================================
// copyright 2025 by nonresistant.near