// 🧊 iced_playground
// src/pages/home.rs
use iced::widget::{column, container, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {
    let home_content = column![
        text("ICED PLAYGROUND").size(24).style(text::primary)
    ]
    .spacing(10)
    .align_x(iced::Alignment::Center);

    container(home_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}


// ==================================
// copyright 2025 by nonresistant.near