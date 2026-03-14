use std::collections::HashMap;
use std::path::PathBuf;

use iced::widget::image::Handle;
use iced_video_player::Video;

#[derive(Debug, Clone)]
pub enum MediaHandle {
    Image(Handle),
    Video(url::Url),
}

pub struct SlideshowPageState {
    pub current_folder: Option<PathBuf>,
    pub files: Vec<(usize, PathBuf)>,
    pub media_handles: HashMap<usize, MediaHandle>,
    pub videos_cache: HashMap<usize, Video>,
    pub nb_preloaded_images: usize,
    pub current_index: usize,
    pub is_playing: bool,
    pub is_randomized: bool,
    pub is_fullscreen: bool,
    pub slideshow_interval_secs: u64,
}

impl SlideshowPageState {
    pub fn new_state() -> Self {
        Self {
            current_folder: None,
            files: Vec::new(),
            media_handles: HashMap::new(),
            videos_cache: HashMap::new(),
            nb_preloaded_images: 10,
            current_index: 0,
            is_playing: false,
            is_fullscreen: false,
            is_randomized: false,
            slideshow_interval_secs: 2500,
        }
    }
}
