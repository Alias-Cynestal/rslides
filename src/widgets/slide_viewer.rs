use iced::{Center, Element, Fill};
use iced::widget::{container, image, text, Column};
use iced_aw::Spinner;

pub fn new<'a>(app_state: &'_ crate::app::RSlidesState) -> Element<'_, crate::app::Message> {
    if let Some(_current_folder) = &app_state.current_folder {
        if !app_state.images.is_empty() {
            return if let Some(handle) = app_state.images_handles.get(&app_state.current_index) {
                container(image(handle))
                    .center_x(Fill)
                    .center_y(Fill)
                    .into()
            } else {
                let current_image_path = &app_state.images[app_state.current_index];
                container(image(current_image_path))
                    .center_x(Fill)
                    .center_y(Fill)
                    .into()
            }
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
    .center_x(Fill)
    .center_y(Fill)
    .into()

}