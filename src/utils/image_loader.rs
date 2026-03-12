use std::path::PathBuf;
use iced::advanced::image::Handle;

pub(crate) async fn load_image_async(index: usize, path: PathBuf) -> (usize, Handle) {
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

    (index, handle)
}