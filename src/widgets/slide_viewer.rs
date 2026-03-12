use iced::{Background, Center, Color, ContentFit, Element, Fill, Length};
use iced::widget::{container, image, text, Column, Container};
use iced_aw::Spinner;

pub fn new<'a>(app_state: &'_ crate::app::RSlidesState) -> Element<'_, crate::app::Message> {
    if let Some(_current_folder) = &app_state.current_folder {
        if !app_state.images.is_empty() {
            let image = if let Some(handle) = app_state.images_handles.get(&app_state.current_index) {
                image(handle)
            } else {
                let current_image_path = &app_state.images[app_state.current_index].1;
                image(current_image_path)
            }
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(ContentFit::Contain);

            return container(image)
                .width(Fill)
                .height(Fill)
                .center(Fill)
                .style(|_x| container::Style {
                    background: Some(Background::Color(Color::from_rgb8(0,0,0))),
                    ..Default::default()
                })
                .into()
        }
    }
    container(Column::new()
        .spacing(20)
        .push(Spinner::new()
            .width(100)
            .height(100)
            .circle_radius(8.0)
        )
        .push(text("Waiting for slides...")
            .size(30)
        )
        .align_x(Center)
    )
    .style(|_x| container::Style {
        background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
        ..Default::default()
    })
    .center(Fill)
    .into()

}