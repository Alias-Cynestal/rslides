mod app;
mod ui;
mod widgets;

use iced::{window};
use app::RSlides;

pub fn main() -> iced::Result {
     iced::application(RSlides::new, RSlides::update, RSlides::view).window(window::Settings{
          size: Default::default(),
          maximized: false,
          fullscreen: true,
          position: Default::default(),
          min_size: None,
          max_size: None,
          visible: true,
          resizable: false,
          closeable: false,
          minimizable: false,
          decorations: false,
          transparent: false,
          blur: false,
          level: Default::default(),
          icon: None,
          platform_specific: Default::default(),
          exit_on_close_request: true,
     }).run()
}
