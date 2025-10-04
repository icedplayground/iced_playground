// 🧊 iced_playground
// src/pages/theme_page.rs
use iced::widget::{button, column, container, text, row};
use iced::{Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
}

pub struct ThemePage {
    current_theme: Theme,
}

impl Default for ThemePage {
    fn default() -> Self {
        Self {
            current_theme: Theme::Dark,
        }
    }
}

impl ThemePage {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ThemeSelected(theme) => {
                self.current_theme = theme;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let theme_buttons = row![
            theme_button("Light", Theme::Light),
            theme_button("Dark", Theme::Dark),
            theme_button("Dracula", Theme::Dracula),
            theme_button("Nord", Theme::Nord),
            theme_button("Solarized Light", Theme::SolarizedLight),
            theme_button("Solarized Dark", Theme::SolarizedDark),
        ]
        .spacing(10);

        let content = column![
            text("Theme Selection").size(24),
            text("Select a theme for the application:").size(16),
            theme_buttons,
            text(format!("Current chosen theme: {:?}", self.current_theme)).size(14)
        ]
        .spacing(20)
        .padding(20)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn theme_button(label: &str, theme: Theme) -> Element<'_, Message> {
    button(text(label))
        .width(Length::Fixed(150.0))
        .padding(10)
        .on_press(Message::ThemeSelected(theme))
        .into()
}