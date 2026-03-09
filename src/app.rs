use std::path::PathBuf;
use iced::Element;
use crate::ui;

pub enum Message {
    NextSlide,
    PreviousSlide,
}

#[derive(Debug)]
pub struct RSlidesState {
    current_folder: Option<PathBuf>,
    images: Vec<PathBuf>,
    current_index: usize,
    is_playing: bool,
    slideshow_interval_secs: u64,
    error_message: Option<String>,
}

#[derive(Debug)]
pub struct RSlides {
    app_state: RSlidesState,
}

impl RSlides {
    pub fn new() -> Self {
        Self {
            app_state : RSlidesState {
                current_folder: None,
                images: Vec::new(),
                current_index: 0,
                is_playing: false,
                slideshow_interval_secs: 5,
                error_message: None,
            },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NextSlide => todo!(),
            Message::PreviousSlide => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        ui::view(&self.app_state)
    }
}