use std::collections::HashMap;
use std::path::PathBuf;
use iced::{event, keyboard, Element, Event, Task};
use iced::keyboard::key::Named;
use iced::widget::image::Handle;
use crate::ui;
use iced::Subscription;
use nfd2::Response;
use crate::utils::handle_slideshow::{get_next_slide, get_previous_slide, randomize_slides, reset_slide_order};
use crate::utils::open_folder::{load_folder, select_folder};
use crate::utils::image_loader::load_image_async;

#[derive(Clone)]
pub enum Message {
    NextSlide,
    PreviousSlide,
    PlaySlideshow,
    PauseSlideshow,
    TogglePlayPause,
    OpenFolder,
    RandomizeSlides,
    ResetSlideOrder,
    ImageLoaded(usize, Handle),
    FolderSelected(Response),
    FullscreenToggle(bool),
    Exit,
}

pub(crate) struct RSlidesState {
    pub current_folder: Option<PathBuf>,
    pub images: Vec<(usize, PathBuf)>, // Store index to return to original order if needed
    pub images_handles: HashMap<usize, Handle>,
    pub nb_preloaded_images: usize,
    pub current_index: usize,
    pub is_playing: bool,
    pub is_randomized: bool,
    pub is_fullscreen: bool,
    pub slideshow_interval_secs: u64,
}

pub struct RSlides {
    app_state: RSlidesState,
}

impl RSlides {
    pub fn new() -> Self {
        let app_state = RSlidesState {
            current_folder: None,
            images: Vec::new(),
            images_handles: HashMap::new(),
            nb_preloaded_images: 10,
            current_index: 0,
            is_playing: false,
            is_fullscreen: false,
            is_randomized: false,
            slideshow_interval_secs: 2500,
        };
        Self {
            app_state,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NextSlide => {
                get_next_slide(&mut self.app_state);
                return self.update_handles_async(true);
            },
            Message::PreviousSlide => {
                get_previous_slide(&mut self.app_state);
                return self.update_handles_async(false);
            },
            Message::PlaySlideshow => {
                self.app_state.is_playing = true;
            },
            Message::PauseSlideshow => {
                self.app_state.is_playing = false;
            },
            Message::TogglePlayPause => {
                self.app_state.is_playing = !self.app_state.is_playing;
            }
            Message::OpenFolder => {
                return select_folder()
            },
            Message::FolderSelected(response) => {
                load_folder(&mut self.app_state, response);
                if (self.app_state.is_randomized) {
                    randomize_slides(&mut self.app_state);
                }
                return self.preload_handles_async();
            },
            Message::ImageLoaded(index, handle) => {
                self.app_state.images_handles.insert(index, handle);
            },
            Message::Exit => std::process::exit(0),
            Message::RandomizeSlides => {
                self.app_state.is_randomized = true;
                randomize_slides(&mut self.app_state);
                return self.preload_handles_async();
            }
            Message::ResetSlideOrder => {
                self.app_state.is_randomized = false;
                reset_slide_order(&mut self.app_state);
                return self.preload_handles_async();
            },
            Message::FullscreenToggle(is_fullscreen) => {
                self.app_state.is_fullscreen = is_fullscreen;
            }
        }
        Task::none()
    }

    fn preload_handles_async(&mut self) -> Task<Message> {
        self.app_state.images_handles.clear();

        let current = self.app_state.current_index;
        let total = self.app_state.images.len();
        let preload_range = self.app_state.nb_preloaded_images;

        if total == 0 {
            return Task::none();
        }

        let start = if current >= preload_range {
            current - preload_range
        } else {
            total.saturating_sub(preload_range - current)
        };

        let end = (current + preload_range) % total;

        // Collect indices to load
        let mut indices_to_load = Vec::new();
        if start <= end {
            for i in start..=end {
                indices_to_load.push(i);
            }
        } else {
            for i in start..total {
                indices_to_load.push(i);
            }
            for i in 0..=end {
                indices_to_load.push(i);
            }
        }

        let tasks: Vec<Task<Message>> = indices_to_load
            .into_iter()
            .filter(|&i| !self.app_state.images_handles.contains_key(&i))
            .filter_map(|i| {
                self.app_state.images.get(i).map(|path| {
                    let path = path.clone();
                    Task::perform(
                        load_image_async(i, path.1),
                        |(index, handle)| Message::ImageLoaded(index, handle)
                    )
                })
            })
            .collect();

        Task::batch(tasks)
    }

    fn update_handles_async(&mut self, is_moving_forward: bool) -> Task<Message> {
        let current = self.app_state.current_index;
        let total = self.app_state.images.len();
        let preload_range = self.app_state.nb_preloaded_images;
        let max_handles = 2 * preload_range + 1;

        if total == 0 {
            return Task::none();
        }
        let index_to_load = if is_moving_forward {
            let next_to_load = (current + preload_range) % total;
            let oldest_relevant = if current >= preload_range {
                current - preload_range
            } else {
                total.saturating_sub(preload_range - current)
            };
            if self.app_state.images_handles.len() > max_handles {
                self.app_state.images_handles.remove(&oldest_relevant);
            }
            next_to_load
        } else {
            let prev_to_load = if current >= preload_range {
                current - preload_range
            } else {
                total.saturating_sub(preload_range - current)
            };
            let newest_irrelevant = (current + preload_range) % total;
            if self.app_state.images_handles.len() > max_handles {
                self.app_state.images_handles.remove(&newest_irrelevant);
            }
            prev_to_load
        };

        if self.app_state.images_handles.contains_key(&index_to_load) {
            return Task::none();
        }
        if let Some(path) = self.app_state.images.get(index_to_load) {
            let path = path.clone();
            Task::perform(
                load_image_async(index_to_load, path.1),
                |(index, handle)| Message::ImageLoaded(index, handle)
            )
        } else {
            Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::view(&self.app_state)
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.timer_subscription(),
            self.keyboard_subscription(),
        ])
    }

    fn timer_subscription(&self) -> Subscription<Message> {
        if self.app_state.is_playing {
            iced::time::every(std::time::Duration::from_millis(self.app_state.slideshow_interval_secs))
                .map(|_| Message::NextSlide)
        } else {
            Subscription::none()
        }
    }

    fn keyboard_subscription(&self) -> Subscription<Message> {
        event::listen_with(move |event, _, _| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(Named::ArrowRight) , .. }) => Some(Message::NextSlide),
                Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(Named::ArrowLeft) , .. }) => Some(Message::PreviousSlide),
                Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(Named::Space) , .. }) => Some(Message::TogglePlayPause),
                Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(Named::Escape) , ..}) => Some(Message::FullscreenToggle(false)),
                _ => None,
            }
        })
    }
}

