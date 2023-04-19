use crate::utils::folder_searcher::folder_searcher::get_all_files_inside_folder;
use std::fmt::Debug;

pub struct Exception {
    pub target_trait: String,
    pub target_trait_render_order: u32,
    pub matching_files: Vec<String>,
    pub matching_files_render_order: u32,
}

impl Debug for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Exception")
            .field("target_trait", &self.target_trait)
            .field("target_trait_render_order", &self.target_trait_render_order)
            .field("matching_files", &self.matching_files)
            .finish()
    }
}

impl Exception {
    pub fn new(
        target_trait: String,
        target_trait_render_order: u32,
        matching_files: Vec<String>,
        matching_files_render_order: u32,
    ) -> Self {
        Self {
            target_trait,
            target_trait_render_order,
            matching_files,
            matching_files_render_order,
        }
    }
}

pub fn get_exceptions() -> Vec<Exception> {
    let mut exceptions: Vec<Exception> = Vec::new();
    return exceptions;
    let all_eye_colors = get_all_files_inside_folder("layers/Eye color");
    let all_eyes_without_cyan = all_eye_colors
        .iter()
        .filter(|file_name| !file_name.contains("Cyan"))
        .map(|file_name| file_name.to_string())
        .collect();
    let high_lid_no_cyan = Exception::new(
        "layers/Background\\Black#1.png".to_string(),
        1,
        all_eyes_without_cyan,
        2,
    );
    exceptions.push(high_lid_no_cyan);
    exceptions
}
