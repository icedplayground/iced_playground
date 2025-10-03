// 🧊 iced playgrount
// src/main.rs
mod home;
mod text_page;
mod button_page;
mod counter_page;

use iced::application;
use iced::widget::{container, row};
use iced::{Element, Result, Task};

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavItem),
    CounterPage(counter_page::Message),
    ButtonPage(button_page::Message),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavItem {
    Home,
    Text,
    Button,
    Counter,
}

pub struct IcedPlayground {
    current_page: NavItem,
    counter_page: counter_page::CounterPage,
    button_page: button_page::ButtonPage,
}

impl Default for IcedPlayground {
    fn default() -> Self {
        Self {
            current_page: NavItem::Home,
            counter_page: counter_page::CounterPage::default(),
            button_page: button_page::ButtonPage::default(),
        }
    }
}

fn main() -> Result {
    application(
        IcedPlayground::title,
        IcedPlayground::update,
        IcedPlayground::view,
    )
    .run_with(|| (IcedPlayground::default(), Task::none()))
}

impl IcedPlayground {
    fn title(&self) -> String {
        String::from("Iced Playground")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(nav_item) => {
                self.current_page = nav_item;
                Task::none()
            }
            Message::CounterPage(msg) => {
                self.counter_page.update(msg);
                Task::none()
            }
            Message::ButtonPage(msg) => {
                self.button_page.update(msg);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = self.sidebar_view();
        let content = self.content_view();

        let layout = row![sidebar, content]
            .spacing(10)
            .padding(10)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);

        container(layout)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .into()
    }

    fn sidebar_view(&self) -> Element<'_, Message> {
        use iced::widget::{button, column, text};

        let nav_items = [
            (NavItem::Home, "HOME"),
            (NavItem::Text, "TEXT"),
            (NavItem::Button, "BUTTON"),
            (NavItem::Counter, "COUNTER"),
        ];

        let mut sidebar_col = column!().spacing(5);

        for (nav_item, label) in nav_items {
            let btn = button(text(label))
                .width(iced::Length::Fill)
                .on_press(Message::Navigation(nav_item));

            // Highlight the active page
            let btn = if self.current_page == nav_item {
                btn.style(iced::widget::button::primary)
            } else {
                btn
            };

            sidebar_col = sidebar_col.push(btn);
        }

        container(sidebar_col)
            .width(iced::Length::Fixed(150.0))
            .padding(10)
            .into()
    }

    fn content_view(&self) -> Element<'_, Message> {
        match self.current_page {
            NavItem::Home => home::view().map(|_| Message::Navigation(NavItem::Home)),
            NavItem::Text => text_page::view().map(|_| Message::Navigation(NavItem::Text)),
            NavItem::Button => self.button_page.view().map(Message::ButtonPage),
            NavItem::Counter => self.counter_page.view().map(Message::CounterPage),
        }
    }
}

// ==================================
// copyright 2025 by nonresistant.near
