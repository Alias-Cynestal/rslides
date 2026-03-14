use iced::{Element, Subscription, Task};

use crate::widgets::slideshow_page::{SlideshowMessage, SlideshowPage};

#[derive(Clone)]
pub enum Message {
    Slideshow(SlideshowMessage),
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum Pages {
    SlideshowPage,
}

pub struct AppState {
    pub current_page: Pages,
    pub slideshow_page: SlideshowPage,
}

pub struct RSlides {
    app_state: AppState,
}

impl RSlides {
    pub fn new() -> Self {
        let app_state = AppState {
            current_page: Pages::SlideshowPage,
            slideshow_page: SlideshowPage::new(),
        };

        Self { app_state }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => std::process::exit(0),
            Message::Slideshow(message) => {
                SlideshowPage::update(&mut self.app_state.slideshow_page, message)
                    .map(|message| Message::Slideshow(message))
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.view()
                .map(Message::Slideshow),
        }
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.subscriptions()
                .map(|message| Message::Slideshow(message)),
        }
    }

    pub fn title(&self) -> String {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.title(),
        }
    }
}