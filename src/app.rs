use std::path::PathBuf;
use iced::Element;
use crate::ui;
use std::fs::read_dir;
use crate::utils::handle_slideshow::{get_next_slide, get_previous_slide};
use crate::utils::open_folder::select_folder;

#[derive(Clone)]
pub enum Message {
    NextSlide,
    PreviousSlide,
    PlaySlideshow,
    PauseSlideshow,
    OpenFolder,
    Exit
}

#[derive(Debug)]
pub(crate) struct RSlidesState {
    pub current_folder: Option<PathBuf>,
    pub images: Vec<PathBuf>,
    pub current_index: usize,
    pub is_playing: bool,
    pub slideshow_interval_secs: u64,
    pub error_message: Option<String>,
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
            Message::NextSlide => get_next_slide(&mut self.app_state),
            Message::PreviousSlide => get_previous_slide(&mut self.app_state),
            Message::PlaySlideshow => {
                self.app_state.is_playing = true;
            },
            Message::PauseSlideshow => {
                self.app_state.is_playing = false;
            },
            Message::OpenFolder => select_folder(&mut self.app_state),
            Message::Exit => std::process::exit(0),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::view(&self.app_state)
    }
}