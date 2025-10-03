// 🧊 iced_playground
// src/text_page.rs
use iced::widget::{column, container, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {
    let code_block = r#"use iced::widget::text;

fn view() -> Element<'static, Message> {
    text("Hello, World!").into()
}"#;

    let text_content = column![
        text("Hello World Page").size(24),
        text("This page demonstrates the text widget in Iced.").size(16),
        text("Hello, World!").size(18),
        text("Code Example:").size(18),
        text(code_block).size(12),
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