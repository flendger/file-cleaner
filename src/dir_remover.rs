use std::fs;
use std::path::Path;

pub fn remove_dir(path: &Path) -> std::io::Result<()> {
    fs::remove_dir(path)
}