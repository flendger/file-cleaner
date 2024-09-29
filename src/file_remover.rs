use std::path::Path;
use std::{fs, io};

pub fn remove_file(path: &Path) -> io::Result<()> {
    fs::remove_file(path)
}