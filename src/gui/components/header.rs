//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, Container, Row, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{quit_analysis_translation, settings_translation};
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

pub fn header(
    font: Font,
    color_gradient: GradientType,
    back_button: bool,
    language: Language,
    last_opened_setting: SettingsPage,
) -> Container<'static, Message, Renderer<StyleType>> {
    let logo = Icon::Sniffnet
        .to_text()
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .width(FillPortion(6))
        .height(Length::Fill)
        .line_height(LineHeight::Relative(1.0))
        .size(100);

    Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(if back_button {
                Container::new(get_button_reset(font, language))
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center)
            } else {
                Container::new(Row::new())
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center)
            })
            .push(logo)
            .push(
                Container::new(get_button_settings(font, language, last_opened_setting))
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
    .height(Length::Fixed(95.0))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(ContainerType::Gradient(color_gradient))
}

fn get_button_reset(
    font: Font,
    language: Language,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Icon::ArrowBack
            .to_text()
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(60.0))
    .on_press(Message::ResetButtonPressed);

    Tooltip::new(
        content,
        quit_analysis_translation(language),
        Position::Right,
    )
    .gap(5)
    .font(font)
    .style(ContainerType::Tooltip)
}

pub fn get_button_settings(
    font: Font,
    language: Language,
    open_overlay: SettingsPage,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Icon::Settings
            .to_text()
            .size(20.5)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(60.0))
    .on_press(Message::OpenSettings(open_overlay));

    Tooltip::new(content, settings_translation(language), Position::Left)
        .gap(5)
        .font(font)
        .style(ContainerType::Tooltip)
}
