// 🧊 iced_playground
// src/pages/theme_page.rs
use iced::widget::{column, container, text, pick_list};
use iced::{Element, Length, Theme};

// Define a helper type to represent theme options with their display labels
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeOption {
    pub name: String,
    pub theme: Theme,
}

impl std::fmt::Display for ThemeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

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

fn get_theme_options() -> Vec<ThemeOption> {
    vec![
        ThemeOption { name: "Light".to_string(), theme: Theme::Light },
        ThemeOption { name: "Dark".to_string(), theme: Theme::Dark },
        ThemeOption { name: "Dracula".to_string(), theme: Theme::Dracula },
        ThemeOption { name: "Nord".to_string(), theme: Theme::Nord },
        ThemeOption { name: "Solarized Light".to_string(), theme: Theme::SolarizedLight },
        ThemeOption { name: "Solarized Dark".to_string(), theme: Theme::SolarizedDark },
        ThemeOption { name: "Gruvbox Light".to_string(), theme: Theme::GruvboxLight },
        ThemeOption { name: "Gruvbox Dark".to_string(), theme: Theme::GruvboxDark },
        ThemeOption { name: "Catppuccin Latte".to_string(), theme: Theme::CatppuccinLatte },
        ThemeOption { name: "Catppuccin Frappe".to_string(), theme: Theme::CatppuccinFrappe },
        ThemeOption { name: "Catppuccin Macchiato".to_string(), theme: Theme::CatppuccinMacchiato },
        ThemeOption { name: "Catppuccin Mocha".to_string(), theme: Theme::CatppuccinMocha },
        ThemeOption { name: "Tokyo Night".to_string(), theme: Theme::TokyoNight },
        ThemeOption { name: "Tokyo Night Storm".to_string(), theme: Theme::TokyoNightStorm },
        ThemeOption { name: "Tokyo Night Light".to_string(), theme: Theme::TokyoNightLight },
        ThemeOption { name: "Kanagawa Wave".to_string(), theme: Theme::KanagawaWave },
        ThemeOption { name: "Kanagawa Dragon".to_string(), theme: Theme::KanagawaDragon },
        ThemeOption { name: "Kanagawa Lotus".to_string(), theme: Theme::KanagawaLotus },
        ThemeOption { name: "Moonfly".to_string(), theme: Theme::Moonfly },
        ThemeOption { name: "Nightfly".to_string(), theme: Theme::Nightfly },
    ]
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
        
        let theme_options = get_theme_options();
        let current_theme_option = theme_options.iter()
            .find(|option| option.theme == self.current_theme)
            .cloned()
            .unwrap_or_else(|| ThemeOption { name: "Dark".to_string(), theme: Theme::Dark });
        
        let theme_picker = pick_list(
            theme_options,
            Some(current_theme_option),
            |theme_option| Message::ThemeSelected(theme_option.theme),
        )
        .placeholder("Select a theme...");
        
        let centered_picker = container(theme_picker)
            .width(Length::Fixed(200.0))
            .center_x(Length::Fill);
        
        let centered_current_theme = container(text(format!("Current chosen theme: {:?}", self.current_theme)).size(14))
            .width(Length::Fill)
            .center_x(Length::Fill);

        let content = column![
            centered_title,
            centered_subtitle,
            centered_picker,
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

