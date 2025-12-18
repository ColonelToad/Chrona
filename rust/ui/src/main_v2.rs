//! Chrona - Watch-style health assistant
//!
//! Multi-tier display with synthetic sensor data, mock ML, and mock LLM.

use iced::{
    alignment, executor, widget::{button, column, container, row, text, Column, Container},
    Application, Command, Element, Length, Settings, Theme,
};

mod display_mode;
mod tier_engine;

use display_mode::DisplayMode;
use tier_engine::TierEngine;

pub fn main() -> iced::Result {
    ChronaApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 700.0),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct ChronaApp {
    display_mode: DisplayMode,
    mini_engine: TierEngine,
    regular_engine: TierEngine,
    pro_engine: TierEngine,
    llm_response: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    SetDisplayMode(DisplayMode),
    AskLlm(core_types::Tier),
    CloseDialog,
}

impl Application for ChronaApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                display_mode: DisplayMode::Triple,
                mini_engine: TierEngine::new_stub(core_types::Tier::Mini8),
                regular_engine: TierEngine::new_stub(core_types::Tier::Standard16),
                pro_engine: TierEngine::new_stub(core_types::Tier::Pro32),
                llm_response: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Chrona - Watch Health Assistant".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetDisplayMode(mode) => {
                self.display_mode = mode;
            }
            Message::AskLlm(tier) => {
                let response = match tier {
                    core_types::Tier::Mini8 => {
                        self.mini_engine.ask_llm("Why is my heart rate this value?")
                    }
                    core_types::Tier::Standard16 => {
                        self.regular_engine.ask_llm("Why is my heart rate this value?")
                    }
                    core_types::Tier::Pro32 => {
                        self.pro_engine.ask_llm("Why is my heart rate this value?")
                    }
                };
                self.llm_response = Some(response);
            }
            Message::CloseDialog => {
                self.llm_response = None;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Update engines on each view render (simplified polling)
        let mini_val = self.mini_engine.current_value();
        let regular_val = self.regular_engine.current_value();
        let pro_val = self.pro_engine.current_value();

        let mode_controls = row![
            text("Display: ").size(16),
            button(text("Mini (1)"))
                .on_press(Message::SetDisplayMode(DisplayMode::Mini)),
            button(text("Regular (2)"))
                .on_press(Message::SetDisplayMode(DisplayMode::Regular)),
            button(text("Pro (3)"))
                .on_press(Message::SetDisplayMode(DisplayMode::Pro)),
            button(text("Triple (T)"))
                .on_press(Message::SetDisplayMode(DisplayMode::Triple)),
        ]
        .spacing(10)
        .padding(10);

        let tier_displays = match self.display_mode {
            DisplayMode::Mini => {
                row![self.render_tier("Mini 8GB", &mini_val, core_types::Tier::Mini8)]
                    .spacing(20)
                    .padding(20)
            }
            DisplayMode::Regular => {
                row![self.render_tier("Regular 16GB", &regular_val, core_types::Tier::Standard16)]
                    .spacing(20)
                    .padding(20)
            }
            DisplayMode::Pro => {
                row![self.render_tier("Pro 32GB", &pro_val, core_types::Tier::Pro32)]
                    .spacing(20)
                    .padding(20)
            }
            DisplayMode::MiniRegular => {
                row![
                    self.render_tier("Mini 8GB", &mini_val, core_types::Tier::Mini8),
                    self.render_tier("Regular 16GB", &regular_val, core_types::Tier::Standard16)
                ]
                .spacing(20)
                .padding(20)
            }
            DisplayMode::RegularPro => {
                row![
                    self.render_tier("Regular 16GB", &regular_val, core_types::Tier::Standard16),
                    self.render_tier("Pro 32GB", &pro_val, core_types::Tier::Pro32)
                ]
                .spacing(20)
                .padding(20)
            }
            DisplayMode::Triple => {
                row![
                    self.render_tier("Mini 8GB", &mini_val, core_types::Tier::Mini8),
                    self.render_tier("Regular 16GB", &regular_val, core_types::Tier::Standard16),
                    self.render_tier("Pro 32GB", &pro_val, core_types::Tier::Pro32)
                ]
                .spacing(20)
                .padding(20)
            }
        };

        let base = column![mode_controls, tier_displays]
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(response) = &self.llm_response {
            container(
                column![
                    base,
                    container(
                        column![
                            text("AI Response").size(20),
                            text(response.as_str()).size(14),
                            button(text("Close")).on_press(Message::CloseDialog)
                        ]
                        .spacing(10)
                        .padding(20)
                    )
                    .padding(30)
                ]
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            base.into()
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl ChronaApp {
    fn render_tier(&self, title: &str, value: &str, tier: core_types::Tier) -> Element<Message> {
        container(
            column![
                text(title).size(20),
                text(value).size(36),
                text("[chart placeholder]").size(12),
                text("Status: Normal").size(12),
                button(text("Ask AI")).on_press(Message::AskLlm(tier)),
            ]
            .spacing(15)
            .padding(20)
            .align_items(alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
