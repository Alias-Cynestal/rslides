use rand::prelude::*;
use std::time::Duration;

use crate::state::slideshow_page_state::SlideshowPageState;

pub(crate) fn get_next_slide(app_state: &mut SlideshowPageState) {
    if app_state.current_folder.is_some() && !app_state.files.is_empty() {
        handle_end_current_slide_is_video(app_state);
        app_state.current_index = (app_state.current_index + 1) % app_state.files.len();
        handle_start_current_slide_is_video(app_state)
    }
}

pub(crate) fn get_previous_slide(app_state: &mut SlideshowPageState) {
    if app_state.current_folder.is_some() && !app_state.files.is_empty() {
        handle_end_current_slide_is_video(app_state);
        app_state.current_index = if app_state.current_index == 0 {
            app_state.files.len() - 1
        } else {
            app_state.current_index - 1
        };
        handle_start_current_slide_is_video(app_state);
    }
}

pub(crate) fn randomize_slides(app_state: &mut SlideshowPageState) {
    if app_state.current_folder.is_some() && !app_state.files.is_empty() {
        let mut rng = rand::rng();
        app_state.files.shuffle(&mut rng);
        app_state.current_index = 0;
    }
}

pub(crate) fn reset_slide_order(app_state: &mut SlideshowPageState) {
    if app_state.current_folder.is_some() && !app_state.files.is_empty() {
        app_state.files.sort_by_key(|&(index, _)| index);
        app_state.current_index = 0;
    }
}

fn handle_end_current_slide_is_video(app_state: &mut SlideshowPageState) {
    if let Some(video) = app_state.videos_cache.get_mut(&app_state.current_index) {
        video.set_paused(true);
        let _ = video.seek(Duration::ZERO, true);
    }
}

fn handle_start_current_slide_is_video(app_state: &mut SlideshowPageState) {
    if let Some(video) = app_state.videos_cache.get_mut(&app_state.current_index) {
        video.set_paused(false);
    }
}
