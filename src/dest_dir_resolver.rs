use chrono::{DateTime, Local};
use std::path::Path;

pub fn resolve_dest(dest_dir_initial: &String, c_date: &DateTime<Local>) -> Box<Path> {
    let mut dest_dir = String::from(dest_dir_initial);
    dest_dir.push_str("/");
    dest_dir.push_str(c_date.format("%Y%m%d").to_string().as_str());

    Box::from(Path::new(&dest_dir))
}
