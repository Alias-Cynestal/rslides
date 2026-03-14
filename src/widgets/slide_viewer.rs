use iced::{Background, Center, Color, ContentFit, Element, Fill, Renderer, Theme};
use iced::widget::{container, image, text, Column};
use iced_aw::Spinner;
use iced_video_player::VideoPlayer;
use crate::state::slideshow_page_state::{MediaHandle, SlideshowPageState};
use crate::widgets::slideshow_page::SlideshowMessage;

pub fn new(app_state: &SlideshowPageState) -> Element<'_, SlideshowMessage> {
    if let Some(_current_folder) = &app_state.current_folder {
        if !app_state.files.is_empty() {
            match app_state.media_handles.get(&app_state.current_index) {
                Some(MediaHandle::Video(_)) => {
                    if let Some(video) = app_state.videos_cache.get(&app_state.current_index) {
                        let media_player: VideoPlayer<'_, SlideshowMessage, Theme, Renderer> =
                            VideoPlayer::new(video)
                                .width(Fill)
                                .height(Fill);
                        return container(media_player)
                            .width(Fill)
                            .height(Fill)
                            .center(Fill)
                            .style(|_x| container::Style {
                                background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                ..Default::default()
                            })
                            .into();
                    }

                    return container(
                        Column::new()
                            .spacing(20)
                            .push(Spinner::new().width(100).height(100).circle_radius(8.0))
                            .push(text("Loading video...").size(30))
                            .align_x(Center),
                    )
                    .style(|_x| container::Style {
                        background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                        ..Default::default()
                    })
                    .center(Fill)
                    .into();
                }
                Some(MediaHandle::Image(handle)) => {
                    let image = if let Some(_handle) = app_state.media_handles.get(&app_state.current_index)
                    {
                        image(handle)
                    } else {
                        let current_image_path = &app_state.files[app_state.current_index].1;
                        image(current_image_path)
                    }
                    .width(Fill)
                    .height(Fill)
                    .content_fit(ContentFit::Contain);

                    return container(image)
                        .width(Fill)
                        .height(Fill)
                        .center(Fill)
                        .style(|_x| container::Style {
                            background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                            ..Default::default()
                        })
                        .into();
                }
                _ => {}
            }
        }
    }

    container(
        Column::new()
            .spacing(20)
            .push(Spinner::new().width(100).height(100).circle_radius(8.0))
            .push(text("Waiting for slides...").size(30))
            .align_x(Center),
    )
    .style(|_x| container::Style {
        background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
        ..Default::default()
    })
    .center(Fill)
    .into()
}