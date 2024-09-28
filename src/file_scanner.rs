use crate::settings::Settings;
use chrono::{DateTime, Days, Local};
use std::ops::Sub;
use std::path::{Path, PathBuf};

pub fn scan(settings: &Settings) -> Vec<Box<Path>> {
    let initial_date = resolve_date(settings);

    let path = Path::new(&settings.folder);
    path.read_dir()
        .unwrap()
        .map(|dir| dir.unwrap())
        .map(|dir| dir.path())
        .filter(|path| path.is_file())
        .filter(|p| initial_date > get_file_date(&p))
        .map(|p| p.into_boxed_path())
        .collect()
}

fn resolve_date(settings: &Settings) -> DateTime<Local> {
    Local::now().sub(Days::new(settings.depth))
}

fn get_file_date(path_buf: &PathBuf) -> DateTime<Local> {
    let f_time = path_buf
        .metadata().unwrap()
        .modified().unwrap();

    DateTime::<Local>::from(f_time)
}