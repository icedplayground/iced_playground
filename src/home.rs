// 🧊 iced_playground
// src/home.rs
use iced::widget::{column, container, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view() -> Element<'static, Message> {
    let home_content = column![
        text("Welcome to Iced Playground").size(24),
        text("This is the home page of your Iced application.").size(16),
        text("Use the sidebar to navigate to different examples:").size(14),
        text("- Hello World: Shows a text widget example").size(14),
        text("- Button: Shows a button widget example").size(14),
        text("- Counter: Shows a counter widget example").size(14),
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