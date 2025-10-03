// 🧊 iced_playground
// src/text_page.rs
use iced::widget::{column, container, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {

    let text_content = column![
        text("TEXT").size(24),
        text("Hello, World!").size(18)
    ]
    .spacing(10)
    .align_x(iced::Alignment::Center);

    container(text_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}