use iced::{color, Alignment, Element, Fill, Renderer, Theme};
use iced::widget::{button, row, Row, Text};
use iced_font_awesome::fa_icon_solid;
use crate::widgets::slideshow_page::SlideshowMessage;

pub fn new(is_playing: &bool, is_randomized: &bool) -> Element<'static, SlideshowMessage> {
    let app_title = Text::new("RSlides")
        .size(50)
        .align_x(Alignment::Start);

    let previous_button: iced::widget::Button<'_, _, Theme, Renderer> =
        button(fa_icon_solid("arrow-left").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::PreviousSlide);

    let play_pause_button: iced::widget::Button<'_, _, Theme, Renderer> = match is_playing {
        true => button(fa_icon_solid("pause").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::PauseSlideshow),
        false => button(fa_icon_solid("play").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::PlaySlideshow),
    };

    let random_button: iced::widget::Button<'_, _, Theme, Renderer> = match is_randomized {
        true => button(fa_icon_solid("shuffle").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::ResetSlideOrder),
        false => button(fa_icon_solid("shuffle").size(20.0).color(color!(0, 0, 0)))
            .on_press(SlideshowMessage::RandomizeSlides),
    };

    let next_button: iced::widget::Button<'_, _, Theme, Renderer> =
        button(fa_icon_solid("arrow-right").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::NextSlide);

    let open_folder_button: iced::widget::Button<'_, _, Theme, Renderer> =
        button(fa_icon_solid("folder-open").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::OpenFolder);

    let fullscreen_button: iced::widget::Button<'_, _, Theme, Renderer> =
        button(fa_icon_solid("expand").size(20.0).color(color!(255, 255, 255)))
            .on_press(SlideshowMessage::FullscreenToggle(true));

    let controls = row!(
        previous_button,
        play_pause_button,
        next_button,
        random_button,
        fullscreen_button,
        open_folder_button,
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