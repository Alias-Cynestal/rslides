
use iced::{Center, Element, Fill};
use iced::widget::{container, image, text, Column};
use iced_aw::Spinner;

pub fn new<'a>(app_state: &crate::app::RSlidesState) -> Element<crate::app::Message> {
    if let Some(current_folder) = &app_state.current_folder {
        if !app_state.images.is_empty() {
            let current_image_path = &app_state.images[app_state.current_index];
            return image(current_image_path).into();
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