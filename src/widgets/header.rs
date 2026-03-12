use iced::{color, Alignment, Element, Fill, Renderer, Theme};
use iced::widget::{button, row, Row, Text};
use iced_font_awesome::{fa_icon_solid};
use crate::app::Message;

pub fn new(is_playing: &bool, is_randomized: &bool) -> Element<'static, Message> {
    let app_title = Text::new("RSlides").
        size(50)
        .align_x(Alignment::Start);

    let previous_button: iced::widget::Button<'_, _, Theme, Renderer> = button(
            fa_icon_solid("arrow-left").size(20.0).color(color!(255, 255, 255))
        ).on_press(Message::PreviousSlide);

    let play_pause_button: iced::widget::Button<'_, _, Theme, Renderer> = match is_playing {
        true => button(
            fa_icon_solid("pause").size(20.0).color(color!(255, 255, 255))
        ).on_press(Message::PauseSlideshow),
        false => button(
            fa_icon_solid("play").size(20.0).color(color!(255, 255, 255))
        ).on_press(Message::PlaySlideshow),
    };

    let random_button: iced::widget::Button<'_, _, Theme, Renderer> = match is_randomized  {
        true => button(
            fa_icon_solid("shuffle").size(20.0).color(color!(255, 255, 255))
        ).on_press(Message::ResetSlideOrder),
        false => button(
            fa_icon_solid("shuffle").size(20.0).color(color!(0, 0, 0))
        ).on_press(Message::RandomizeSlides),
    };

    let next_button: iced::widget::Button<'_, _, Theme, Renderer> = button(
        fa_icon_solid("arrow-right").size(20.0).color(color!(255, 255, 255))
    ).on_press(Message::NextSlide);

    let open_folder_button: iced::widget::Button<'_, _, Theme, Renderer> = button(
        fa_icon_solid("folder-open").size(20.0).color(color!(255, 255, 255))
    ).on_press(Message::OpenFolder);

    let fullscreen_button: iced::widget::Button<'_, _, Theme, Renderer> = button(
        fa_icon_solid("expand").size(20.0).color(color!(255, 255, 255))
    ).on_press(Message::FullscreenToggle(true));

    let exit_button: iced::widget::Button<'_, _, Theme, Renderer> = button(
        fa_icon_solid("xmark").size(20.0).color(color!(255, 255, 255))
    ).on_press(Message::Exit);

    let controls = row!(
        previous_button,
        play_pause_button,
        next_button,
        random_button,
        fullscreen_button,
        open_folder_button,
        exit_button
    )
        .spacing(20)
        .padding(10)
        .align_y(Alignment::Center);
    Row::new()
        .push(app_title)
        .push(controls)
        .width(Fill)
        .spacing(1975)
        .align_y(Alignment::Center)
        .into()
}