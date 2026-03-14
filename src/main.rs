#![windows_subsystem = "windows"]

mod app;
mod state;
mod widgets;
pub mod utils;

use std::env;
use std::path::PathBuf;

use app::RSlides;
use iced::window;

pub fn main() -> iced::Result {
    configure_gstreamer_runtime();

    iced::application(RSlides::new, RSlides::update, RSlides::view)
        .window(window::Settings {
            size: Default::default(),
            maximized: false,
            fullscreen: true,
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            closeable: false,
            minimizable: false,
            decorations: false,
            transparent: false,
            blur: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        })
        .title(RSlides::title)
        .subscription(RSlides::subscriptions)
        .run()
}


fn configure_gstreamer_runtime() {
    if !cfg!(target_os = "windows") {
        return;
    }

    let exe_dir: PathBuf = match env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
    {
        Some(dir) => dir,
        None => return,
    };

    let plugin_dir = exe_dir.join("gstreamer-1.0");
    let scanner = exe_dir.join("gst-plugin-scanner.exe");

    if plugin_dir.is_dir() {
        // In Rust 2024 this is unsafe; we do it once at startup before worker threads.
        unsafe {
            env::set_var("GST_PLUGIN_PATH_1_0", &plugin_dir);
            env::set_var("GST_PLUGIN_SYSTEM_PATH_1_0", &plugin_dir);
        }
    }

    if scanner.is_file() {
        // Same startup-only environment mutation rule as above.
        unsafe {
            env::set_var("GST_PLUGIN_SCANNER", scanner);
        }
    }
}
