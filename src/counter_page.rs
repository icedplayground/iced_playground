// 🧊 iced_playground
// src/counter_page.rs
use iced::widget::{button, column, container, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

pub struct CounterPage {
    count: i32,
}

impl Default for CounterPage {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl CounterPage {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.count += 1;
            }
            Message::Decrement => {
                self.count -= 1;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let code_block = r#"use iced::widget::{button, column, text};

let counter = column![
    text(self.count).size(50),
    button(text("+")).on_press(Message::Increment),
    button(text("-")).on_press(Message::Decrement),
]
.spacing(10);"#;

        let counter_content = column![
            text("Counter Page").size(24),
            text("This page demonstrates a counter using Iced widgets.").size(16),
            button(text("+"))
                .on_press(Message::Increment),
            text(self.count).size(50),
            button(text("-"))
                .on_press(Message::Decrement),
            text("Code Example:").size(18),
            text(code_block).size(12),
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center);

        container(counter_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}