use chrono::{DateTime, Days, Local};
use std::io::Error;
use std::ops::Sub;
use std::path::{Path, PathBuf};

pub fn scan(folder: &String, depth: u64) -> Result<Vec<Box<Path>>, Error> {
    let initial_date = resolve_date(depth);

    let path = Path::new(&folder);
    let files = path.read_dir()?
        .flat_map(|dir| dir)
        .map(|dir| dir.path())
        .filter(|path| path.is_file())
        .filter(|p| initial_date > get_file_date(&p))
        .map(|p| p.into_boxed_path())
        .collect();

    Ok(files)
}

fn resolve_date(depth: u64) -> DateTime<Local> {
    Local::now().sub(Days::new(depth))
}

fn get_file_date(path_buf: &PathBuf) -> DateTime<Local> {
    let f_time = path_buf
        .metadata().unwrap()
        .modified().unwrap();

    DateTime::<Local>::from(f_time)
}