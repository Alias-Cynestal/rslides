pub(crate) fn get_next_slide(app_state: &mut crate::app::RSlidesState) {
    if let Some(current_folder) = &app_state.current_folder {
        if !app_state.images.is_empty() {
            app_state.current_index = (app_state.current_index + 1) % app_state.images.len();
        }
    }
}

pub(crate) fn get_previous_slide(app_state: &mut crate::app::RSlidesState) {
    if app_state.current_folder.is_some() {
        if !app_state.images.is_empty() {
            app_state.current_index = if app_state.current_index == 0 { app_state.images.len() - 1 }
            else {(app_state.current_index - 1)};
        }
    }
}

pub(crate) fn randomize_slides(app_state: &mut crate::app::RSlidesState) {
    if app_state.current_folder.is_some() {
        if !app_state.images.is_empty() {
            use rand::prelude::*;
            let mut rng = rand::rng();
            app_state.images.shuffle(&mut rng);
            app_state.current_index = 0;
        }
    }
}

pub(crate) fn reset_slide_order(app_state: &mut crate::app::RSlidesState) {
    if app_state.current_folder.is_some() {
        if !app_state.images.is_empty() {
            app_state.images.sort_by_key(|&(index, _)| index);
            app_state.current_index = 0;
        }
    }
}