use iced::Element;
use iced::widget::Column;
use crate::app::{Message, RSlidesState};

use crate::widgets::header;

pub fn view(p0: &'_ RSlidesState) -> Element<'_, Message> {
    Column::new()
        .width(iced::Length::Fill)
        .push(header::new(&p0.is_playing))
        .into()
}