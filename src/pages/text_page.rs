// 🧊 iced_playground
// src/pages/text_page.rs
use iced::widget::{column, container, row, text};
use iced::{ Element, Length};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {
    // Define different text styles using colors
    let base_text = text("BASE").size(16).style(text::base);
    let danger_text = text("DANGER").size(16).style(text::danger);
    let default_text = text("DEFAULT").size(16).style(text::default);
    let primary_text = text("PRIMARY").size(16).style(text::primary);
    let secondary_text = text("SECONDARY").size(16).style(text::secondary);
    let success_text = text("SUCCESS").size(16).style(text::success);

    let text_examples = row![
        base_text,
        danger_text,
        default_text,
        primary_text,
        secondary_text,
        success_text,
    ]
    .spacing(10);

    let content = column![text("TEXT").size(24), text_examples]
        .spacing(10)
        .align_x(iced::Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}


// ==================================
// copyright 2025 by nonresistant.near