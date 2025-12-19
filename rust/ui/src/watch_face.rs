//! California-style watch face for Chrona

use iced::{widget::{canvas, Column, Container, Row, Text, Button}, Alignment, Element, Length, Theme, Color};
use iced::{Border, Shadow};
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::{Renderer, mouse, Point, Rectangle};
use chrono::{Local, Datelike, Timelike};

#[derive(Debug, Clone, Copy)]
pub enum WatchFaceMessage {
    ActivityClicked,
}

pub struct WatchFace {
    pub show_activity: bool,
}

impl WatchFace {
    pub fn view<'a>(&self) -> Element<'a, WatchFaceMessage> {
        let now = Local::now();
        let day = now.format("%A").to_string();
        let date = now.format("%b %e").to_string();
        let time = now.format("%H:%M").to_string();

        // Analog clock as a canvas
        let clock = Canvas::new(AnalogClock { time: now.time() })
            .width(Length::Fixed(220.0))
            .height(Length::Fixed(220.0));

        // Activity complication (yellow circle, clickable)
        let activity = Button::new(
            canvas::Canvas::new(ActivityCircle)
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0))
        )
        .on_press(WatchFaceMessage::ActivityClicked)
        .style(iced::theme::Button::Custom(Box::new(ActivityButtonStyle)));

        let face = Column::new()
            .align_items(Alignment::Center)
            .push(Text::new(day).size(28))
            .push(Text::new(date).size(20))
            .push(clock)
            .push(activity)
            .spacing(10);

        Container::new(face)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

struct AnalogClock {
    time: chrono::NaiveTime,
}

impl<Message> Program<Message> for AnalogClock {
    type State = ();
    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = bounds.width.min(bounds.height) / 2.0 - 8.0;

        // Draw clock face
        frame.fill(&Path::circle(center, radius), Color::from_rgb(0.1, 0.1, 0.1));
        frame.stroke(&Path::circle(center, radius), Stroke::default().with_width(3.0).with_color(Color::WHITE));

        // Draw hour marks
        for i in 0..12 {
            let angle = (i as f32) * std::f32::consts::TAU / 12.0;
            let inner = Point::new(
                center.x + (radius - 12.0) * angle.sin(),
                center.y - (radius - 12.0) * angle.cos(),
            );
            let outer = Point::new(
                center.x + radius * angle.sin(),
                center.y - radius * angle.cos(),
            );
            frame.stroke(&Path::line(inner, outer), Stroke::default().with_width(2.0).with_color(Color::WHITE));
        }

        // Draw hour hand
        let hour = self.time.hour() as f32 % 12.0 + self.time.minute() as f32 / 60.0;
        let hour_angle = hour * std::f32::consts::TAU / 12.0;
        let hour_hand = Point::new(
            center.x + (radius - 48.0) * hour_angle.sin(),
            center.y - (radius - 48.0) * hour_angle.cos(),
        );
        frame.stroke(&Path::line(center, hour_hand), Stroke::default().with_width(6.0).with_color(Color::WHITE));

        // Draw minute hand
        let minute = self.time.minute() as f32 + self.time.second() as f32 / 60.0;
        let min_angle = minute * std::f32::consts::TAU / 60.0;
        let min_hand = Point::new(
            center.x + (radius - 24.0) * min_angle.sin(),
            center.y - (radius - 24.0) * min_angle.cos(),
        );
        frame.stroke(&Path::line(center, min_hand), Stroke::default().with_width(4.0).with_color(Color::from_rgb(1.0, 1.0, 0.0)));

        // Draw second hand
        let sec = self.time.second() as f32;
        let sec_angle = sec * std::f32::consts::TAU / 60.0;
        let sec_hand = Point::new(
            center.x + (radius - 16.0) * sec_angle.sin(),
            center.y - (radius - 16.0) * sec_angle.cos(),
        );
        frame.stroke(&Path::line(center, sec_hand), Stroke::default().with_width(2.0).with_color(Color::from_rgb(1.0, 0.2, 0.2)));

        vec![frame.into_geometry()]
    }
}

struct ActivityCircle;

impl<Message> Program<Message> for ActivityCircle {
    type State = ();
    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = bounds.width.min(bounds.height) / 2.0 - 4.0;
        frame.fill(&Path::circle(center, radius), Color::from_rgb(1.0, 0.85, 0.1));
        frame.stroke(&Path::circle(center, radius), Stroke::default().with_width(2.0).with_color(Color::from_rgb(0.8, 0.7, 0.1)));
        vec![frame.into_geometry()]
    }
}

struct ActivityButtonStyle;

impl iced::widget::button::StyleSheet for ActivityButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
                shadow_offset: iced::Vector::default(),
            text_color: Color::TRANSPARENT,
        }
    }
}
