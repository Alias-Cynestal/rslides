use iced::{Element, Subscription, Task};
use crate::state::slideshow_config_page_state::SlideshowConfigPageState;
use crate::widgets::{header, slide_viewer};

#[derive(Clone)]
pub enum SlideshowConfigMessage {

}

pub struct SlideshowConfigPage {
    state: SlideshowConfigPageState,
}

impl SlideshowConfigPage {
    pub fn new() -> Self {
        SlideshowConfigPage {
            state: SlideshowConfigPageState::new_state(),
        }
    }
    pub fn update(&mut self, message: SlideshowConfigMessage) -> Task<SlideshowConfigMessage> {
        match message {
            _ => {}
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, SlideshowConfigMessage> {
        todo!()
    }

    pub fn subscriptions(&self) -> Subscription<SlideshowConfigMessage> {
        Subscription::batch(vec![

        ])
    }

    pub fn title(&self) -> String {
        "RSlides".to_string()
    }
}