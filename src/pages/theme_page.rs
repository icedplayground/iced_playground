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
        let centered_title = container(text("THEME").size(24))
            .width(Length::Fill)
            .center_x(Length::Fill);
        
        let centered_subtitle = container(text("Select a theme for the application:").size(16))
            .width(Length::Fill)
            .center_x(Length::Fill);
        
        let theme_buttons = column![
            row![
                theme_button("Light", Theme::Light),
                theme_button("Dark", Theme::Dark),
                theme_button("Dracula", Theme::Dracula),
                theme_button("Nord", Theme::Nord),
            ]
            .spacing(10),
            row![
                theme_button("Solarized Light", Theme::SolarizedLight),
                theme_button("Solarized Dark", Theme::SolarizedDark),
                theme_button("GruvboxLight", Theme::GruvboxLight),
                theme_button("GruvboxDark", Theme::GruvboxDark),
            ]
            .spacing(10),
            row![
                theme_button("Catppuccin Latte", Theme::CatppuccinLatte),
                theme_button("Catppuccin Frappe", Theme::CatppuccinFrappe),
                theme_button("Catppuccin Macchiato", Theme::CatppuccinMacchiato),
                theme_button("Catppuccin Mocha", Theme::CatppuccinMocha),
            ]
            .spacing(10),
            row![
                theme_button("Tokyo Night", Theme::TokyoNight),
                theme_button("Tokyo Night Storm", Theme::TokyoNightStorm),
                theme_button("Tokyo Night Light", Theme::TokyoNightLight),
                theme_button("Kanagawa Wave", Theme::KanagawaWave),
            ]
            .spacing(10),
            row![
                theme_button("Kanagawa Dragon", Theme::KanagawaDragon),
                theme_button("Kanagawa Lotus", Theme::KanagawaLotus),
                theme_button("Moonfly", Theme::Moonfly),
                theme_button("Nightfly", Theme::Nightfly),
            ]
            .spacing(10)
        ]
        .spacing(10);
        
        let centered_buttons = container(theme_buttons)
            .width(Length::Shrink)
            .center_x(Length::Fill);
        
        let centered_current_theme = container(text(format!("Current chosen theme: {:?}", self.current_theme)).size(14))
            .width(Length::Fill)
            .center_x(Length::Fill);

        let content = column![
            centered_title,
            centered_subtitle,
            centered_buttons,
            centered_current_theme
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