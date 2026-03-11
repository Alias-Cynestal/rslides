use std::path::PathBuf;
use iced::{Element, Task};
use crate::ui;
use iced::Subscription;
use nfd2::Response;
use crate::utils::handle_slideshow::{get_next_slide, get_previous_slide};
use crate::utils::open_folder::{load_folder, select_folder};

#[derive(Clone)]
pub enum Message {
    NextSlide,
    PreviousSlide,
    PlaySlideshow,
    PauseSlideshow,
    OpenFolder,
    FolderSelected(Response),
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
                slideshow_interval_secs: 2500,
                error_message: None,
            },
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NextSlide => get_next_slide(&mut self.app_state),
            Message::PreviousSlide => get_previous_slide(&mut self.app_state),
            Message::PlaySlideshow => {
                self.app_state.is_playing = true;
            },
            Message::PauseSlideshow => {
                self.app_state.is_playing = false;
            },
            Message::OpenFolder => {
                return select_folder()
            },
            Message::FolderSelected(response) => load_folder(&mut self.app_state, response),
            Message::Exit => std::process::exit(0),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::view(&self.app_state)
    }

    pub fn timer_subscription(&self) -> Subscription<Message> {
        if self.app_state.is_playing {
            iced::time::every(std::time::Duration::from_millis(self.app_state.slideshow_interval_secs))
                .map(|_| Message::NextSlide)
        } else {
            Subscription::none()
        }
    }
}