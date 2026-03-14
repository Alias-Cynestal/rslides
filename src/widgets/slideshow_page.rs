use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use iced::keyboard::key::Named;
use iced::{event, keyboard, Element, Event, Subscription, Task};
use iced_video_player::Video;
use nfd2::Response;
use crate::state::slideshow_page_state::{MediaHandle, SlideshowPageState};
use crate::utils::handle_slideshow::{
    get_next_slide, get_previous_slide, randomize_slides, reset_slide_order,
};
use crate::utils::media_loader::{load_media_async, prepare_video_async};
use crate::utils::open_folder::{load_folder, select_folder};
use crate::widgets::{header, slide_viewer};

#[derive(Clone)]
pub enum SlideshowMessage {
    NextSlide,
    PreviousSlide,
    PlaySlideshow,
    PauseSlideshow,
    TogglePlayPause,
    OpenFolder,
    RandomizeSlides,
    ResetSlideOrder,
    MediaLoaded((usize, MediaHandle)),
    VideoPrepared((usize, url::Url, Arc<Mutex<Option<Video>>>)),
    FolderSelected(Response),
    FullscreenToggle(bool),
    None,
}

pub struct SlideshowPage {
    state: SlideshowPageState,
}

impl SlideshowPage {
    pub fn new() -> Self {
        SlideshowPage {
            state: SlideshowPageState::new_state(),
        }
    }
    pub fn update(&mut self, message: SlideshowMessage) -> Task<SlideshowMessage> {
        match message {
            SlideshowMessage::NextSlide => {
                get_next_slide(&mut self.state);
                return self.update_handles_async(true);
            }
            SlideshowMessage::PreviousSlide => {
                get_previous_slide(&mut self.state);
                return self.update_handles_async(false);
            }
            SlideshowMessage::PlaySlideshow => {
                self.state.is_playing = true;
            }
            SlideshowMessage::PauseSlideshow => {
                self.state.is_playing = false;
            }
            SlideshowMessage::TogglePlayPause => {
                self.state.is_playing = !self.state.is_playing;
            }
            SlideshowMessage::OpenFolder => {
                return select_folder();
            }
            SlideshowMessage::FolderSelected(response) => {
                load_folder(&mut self.state, response);
                if self.state.is_randomized {
                    randomize_slides(&mut self.state);
                }
                return self.preload_handles_async();
            }
            SlideshowMessage::MediaLoaded((index, handle)) => {
                if let MediaHandle::Video(uri) = &handle {
                    let uri = uri.clone();
                    self.state.media_handles.insert(index, handle);
                    return Task::perform(prepare_video_async(uri), move |res| match res {
                        Some((ready_uri, video)) => {
                            SlideshowMessage::VideoPrepared((index, ready_uri, video))
                        }
                        None => SlideshowMessage::None,
                    });
                }
                self.state.media_handles.insert(index, handle);
            }
            SlideshowMessage::VideoPrepared((index, uri, video_slot)) => {
                let is_current_video = matches!(
                    self.state.media_handles.get(&mut self.state.current_index),
                    Some(MediaHandle::Video(current_uri)) if current_uri == &uri
                );

                if let Ok(mut guard) = video_slot.lock() {
                    if let Some(mut video) = guard.take() {
                        if is_current_video {
                            video.set_paused(false);
                        }
                        video.set_looping(true);
                        self.state.videos_cache.insert(index, video);
                    }
                }
            }
            SlideshowMessage::RandomizeSlides => {
                self.state.is_randomized = true;
                randomize_slides(&mut self.state);
                return self.preload_handles_async();
            }
            SlideshowMessage::ResetSlideOrder => {
                self.state.is_randomized = false;
                reset_slide_order(&mut self.state);
                return self.preload_handles_async();
            }
            SlideshowMessage::FullscreenToggle(is_fullscreen) => {
                self.state.is_fullscreen = is_fullscreen;
            }
            _ => {}
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, SlideshowMessage> {
        if self.state.is_fullscreen {
            return slide_viewer::new(&self.state);
        }

        iced::widget::Column::new()
            .width(iced::Length::Fill)
            .push(header::new(&self.state.is_playing, &self.state.is_randomized))
            .push(slide_viewer::new(&self.state))
            .height(iced::Length::Fill)
            .into()
    }

    pub fn subscriptions(&self) -> Subscription<SlideshowMessage> {
        Subscription::batch(vec![
            self.timer_subscription(),
            self.keyboard_subscription(),
        ])
    }

    pub fn title(&self) -> String {
        if let Some(folder) = &self.state.current_folder {
            if let Some(file_name) = folder.file_name() {
                return format!("RSlides - {}", file_name.to_string_lossy());
            }
        }
        "RSlides".to_string()
    }

    fn timer_subscription(&self) -> Subscription<SlideshowMessage> {
        if self.state.is_playing {
            iced::time::every(std::time::Duration::from_millis(
                self.state.slideshow_interval_secs,
            ))
            .map(|_| SlideshowMessage::NextSlide)
        } else {
            Subscription::none()
        }
    }

    fn keyboard_subscription(&self) -> Subscription<SlideshowMessage> {
        event::listen_with(move |event, _, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(Named::ArrowRight),
                ..
            }) => Some(SlideshowMessage::NextSlide),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(Named::ArrowLeft),
                ..
            }) => Some(SlideshowMessage::PreviousSlide),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(Named::Space),
                ..
            }) => Some(SlideshowMessage::TogglePlayPause),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(Named::Escape),
                ..
            }) => Some(SlideshowMessage::FullscreenToggle(false)),
            _ => None,
        })
    }

    fn preload_handles_async(&mut self) -> Task<SlideshowMessage> {
        self.state.media_handles.clear();
        self.state.videos_cache.clear();

        let current = self.state.current_index;
        let total = self.state.files.len();
        let preload_range = self.state.nb_preloaded_images;

        if total == 0 {
            return Task::none();
        }

        let start = if current >= preload_range {
            current - preload_range
        } else {
            total.saturating_sub(preload_range - current)
        };

        let end = (current + preload_range) % total;

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

        let tasks: Vec<Task<SlideshowMessage>> = indices_to_load
            .into_iter()
            .filter(|&i| !self.state.media_handles.contains_key(&i))
            .filter_map(|i| {
                self.state
                    .files
                    .get(i)
                    .map(|path| Self::get_media_load_task(i, path.1.clone()))
            })
            .collect();

        Task::batch(tasks)
    }

    fn update_handles_async(
        &mut self,
        is_moving_forward: bool,
    ) -> Task<SlideshowMessage> {
        let current = self.state.current_index;
        let total = self.state.files.len();
        let preload_range = self.state.nb_preloaded_images;
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
            if self.state.media_handles.len() > max_handles {
                self.state.media_handles.remove(&oldest_relevant);
                self.state.videos_cache.remove(&oldest_relevant);
            }
            next_to_load
        } else {
            let prev_to_load = if current >= preload_range {
                current - preload_range
            } else {
                total.saturating_sub(preload_range - current)
            };
            let newest_irrelevant = (current + preload_range) % total;
            if self.state.media_handles.len() > max_handles {
                self.state.media_handles.remove(&newest_irrelevant);
                self.state.videos_cache.remove(&newest_irrelevant);
            }
            prev_to_load
        };

        if self.state.media_handles.contains_key(&index_to_load) {
            return Task::none();
        }

        if let Some(path) = self.state.files.get(index_to_load) {
            let path = path.clone();
            Self::get_media_load_task(index_to_load, path.1.clone())
        } else {
            Task::none()
        }
    }

    fn get_media_load_task(index: usize, path: PathBuf) -> Task<SlideshowMessage> {
        Task::perform(load_media_async(index, path), move |res| match res {
            Some((index, handle)) => SlideshowMessage::MediaLoaded((index, handle)),
            None => SlideshowMessage::None,
        })
    }
}

