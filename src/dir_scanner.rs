use std::path::Path;
use crate::settings::Settings;

pub fn scan_dirs(settings: &Settings) -> Vec<Box<Path>> {
    let path = Path::new(&settings.folder);

    path.read_dir()
        .unwrap()
        .flat_map(|dir| dir)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|path| path.into_boxed_path())
        .collect()
}