mod app;
mod ui;

use app::RSlides;

pub fn main() -> iced::Result {
     iced::application(RSlides::new, RSlides::update, RSlides::view).run()
}
