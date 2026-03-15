use iced::{Element, Subscription, Task};
use crate::widgets::slideshow_config_page::{SlideshowConfigMessage, SlideshowConfigPage};
use crate::widgets::slideshow_page::{SlideshowMessage, SlideshowPage};

#[derive(Clone)]
pub enum Message {
    Slideshow(SlideshowMessage),
    SlideshowConfig(SlideshowConfigMessage),
    Exit,
}

#[derive(Debug, Clone, Copy)]
enum Pages {
    SlideshowConfigPage,
    SlideshowPage,
}

struct AppState {
    current_page: Pages,
    slideshow_page: SlideshowPage,
    slideshow_config_page: SlideshowConfigPage,
}

pub struct RSlides {
    app_state: AppState,
}

impl RSlides {
    pub fn new() -> Self {
        let app_state = AppState {
            current_page: Pages::SlideshowPage,
            slideshow_page: SlideshowPage::new(),
            slideshow_config_page: SlideshowConfigPage::new(),
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
            Message::SlideshowConfig(message) => {
                SlideshowConfigPage::update(&mut self.app_state.slideshow_config_page, message)
                    .map(|message| Message::SlideshowConfig(message))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.view()
                .map(Message::Slideshow),
            Pages::SlideshowConfigPage => self.app_state.slideshow_config_page.view()
                .map(Message::SlideshowConfig),
        }
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.subscriptions()
                .map(|message| Message::Slideshow(message)),
            Pages::SlideshowConfigPage => self.app_state.slideshow_config_page.subscriptions()
                .map(|message| Message::SlideshowConfig(message)),
        }
    }

    pub fn title(&self) -> String {
        match self.app_state.current_page {
            Pages::SlideshowPage => self.app_state.slideshow_page.title(),
            Pages::SlideshowConfigPage => self.app_state.slideshow_config_page.title(),
        }
    }
}