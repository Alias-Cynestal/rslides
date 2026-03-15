use std::path::PathBuf;
use url::Url;

enum SourceType {
    File(PathBuf),
    Folder(PathBuf),
    Link(Url)
}

pub struct Source {
    source_type: SourceType,
    source_name: String
}

pub struct SlideshowConfigPageState {
    sources: Vec<Source>,
}

impl SlideshowConfigPageState {
    pub fn new_state() -> Self {
        Self {
            sources: Vec::new(),
        }
    }
}
