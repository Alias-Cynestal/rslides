use std::fs::read_dir;
use std::path::PathBuf;

use iced::Task;
use nfd2::Response;

use crate::state::slideshow_page_state::SlideshowPageState;
use crate::widgets::slideshow_page::SlideshowMessage;

pub(crate) fn select_folder() -> Task<SlideshowMessage> {
    Task::perform(open_file_dialog(), SlideshowMessage::FolderSelected)
}

pub(crate) fn load_folder(app_state: &mut SlideshowPageState, response: Response) {
    let mut i = 0;
    match response {
        Response::Okay(folder_path) => {
            reset_slideshow(app_state);
            app_state.current_folder = Some(PathBuf::from(folder_path));
            read_dir(app_state.current_folder.as_ref().unwrap())
                .expect("Failed to read directory")
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| {
                    if let Some(ext) = path.extension() {
                        matches!(
                            ext.to_str().unwrap_or("").to_lowercase().as_str(),
                            "jpg"
                                | "jpeg"
                                | "png"
                                | "bmp"
                                | "gif"
                                | "webp"
                                | "mp4"
                                | "avi"
                                | "mkv"
                        )
                    } else {
                        false
                    }
                })
                .for_each(|path| {
                    app_state.files.push((i, path));
                    i += 1;
                });
        }
        _ => (),
    }
}

async fn open_file_dialog() -> Response {
    nfd2::open_pick_folder(None).expect("Failed to open nfd")
}

fn reset_slideshow(app_state: &mut SlideshowPageState) {
    app_state.current_index = 0;
    app_state.files.clear();
    app_state.current_folder = None;
}