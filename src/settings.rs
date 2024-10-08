use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Settings {
    pub folder: String,
    pub depth: u64,
    pub hierarchy: bool,
    pub skip_root: bool,
}