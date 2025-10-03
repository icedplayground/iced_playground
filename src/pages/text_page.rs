// 🧊 iced_playground
// src/text_page.rs
use iced::widget::{column, container, text};
use iced::{Element, Length, Color};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {
    // Define different text styles using colors
    let base_text = text("BASE").size(16);
    
    let danger_text = text("DANGER")
        .size(16)
        .color(Color::from_rgb(0.9, 0.2, 0.2)); // Red color
    
    let default_text = text("DEFAULT").size(16);
    
    let primary_text = text("PRIMARY")
        .size(16)
        .color(Color::from_rgb(0.2, 0.6, 0.9)); // Blue color
    
    let secondary_text = text("SECONDARY")
        .size(16)
        .color(Color::from_rgb(0.6, 0.6, 0.6)); // Gray color
    
    let success_text = text("SUCCESS")
        .size(16)
        .color(Color::from_rgb(0.2, 0.8, 0.2)); // Green color
    
    let text_examples = column![
        text("TEXT").size(24),
        base_text,
        danger_text,
        default_text,
        primary_text,
        secondary_text,
        success_text,
    ]
    .spacing(10)
    .align_x(iced::Alignment::Start);

    container(text_examples)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
}