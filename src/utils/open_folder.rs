use std::fs::read_dir;
use std::path::PathBuf;

pub fn select_folder(app_state: &mut crate::app::RSlidesState) {
    let result = nfd2::open_pick_folder(None).expect("Failed to open nfd");
    match result {
        nfd2::Response::Okay(folder_path) => {
            app_state.images.clear();
            app_state.current_folder = Some(PathBuf::from(folder_path));
            read_dir(app_state.current_folder.as_ref().unwrap())
                .expect("Failed to read directory")
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| {
                    if let Some(ext) = path.extension() {
                        matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "bmp" | "gif")
                    } else {
                        false
                    }
                })
                .for_each(|path| app_state.images.push(path));
        }
        _ => (),
    }
}