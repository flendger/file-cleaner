use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Settings {
    pub folder: String,
    pub depth: u64,
    pub hierarchy: bool,
    pub skip_root: bool,
    pub cmd: String, // r: remove, m: move, a: archive
    pub dist_dir: String, //using for movement and archiving
}