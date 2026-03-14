use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use iced::advanced::image::Handle;
use iced_video_player::Video;

use crate::state::slideshow_page_state::MediaHandle;

pub(crate) enum MediaType {
    Image,
    Video,
    Other
}

pub async fn load_media_async(index: usize, path: PathBuf) -> Option<(usize, MediaHandle)> {
    match get_media_type(&path) {
        MediaType::Image => {
            let image = load_image_async(index, path).await;
            Some((index, MediaHandle::Image(image)))
        }
        MediaType::Video => {
            Some((index, MediaHandle::Video(url::Url::from_file_path(path).expect("REASON"))))
        }
        _ => { None }
    }
}

async fn load_image_async(_index: usize, path: PathBuf) -> Handle {
    let handle = smol::unblock(move || {
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(img) = image::load_from_memory(&bytes) {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                return Handle::from_rgba(width, height, rgba.into_raw());
            }
        }
        Handle::from_rgba(1, 1, vec![0, 0, 0, 255])
    })
        .await;

    handle
}

pub async fn prepare_video_async(
    uri: url::Url,
) -> Option<(url::Url, Arc<Mutex<Option<Video>>>)> {
    let slot = Arc::new(Mutex::new(None));
    let slot_for_worker = Arc::clone(&slot);
    let uri_for_worker = uri.clone();

    smol::unblock(move || {
        let mut video = Video::new(&uri_for_worker).expect("Failed to create video");
        video.set_looping(true);
        video.set_paused(true);
        if let Ok(mut guard) = slot_for_worker.lock() {
            *guard = Some(video);
        }
    })
    .await;

    Some((uri, slot))
}

pub(crate) fn get_media_type(path: &PathBuf) -> MediaType {
    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" => MediaType::Image,
            "mp4" | "avi" | "mkv" => MediaType::Video,
            _ => MediaType::Other,
        }
    } else {
        MediaType::Other
    }
}