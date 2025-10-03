// 🧊 iced_playground
// src/pages/counter_page.rs
use iced::widget::{button, column, container, row, text};
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
        let counter_buttons = row![
            button(text("-")).on_press(Message::Decrement).padding(15),
            text(self.count).size(50),
            button(text("+")).on_press(Message::Increment).padding(15),
        ]
        .spacing(15)
        .align_y(iced::Alignment::Center);

        let counter_content = column![text("COUNTER").size(24), counter_buttons]
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


// ==================================
// copyright 2025 by nonresistant.near