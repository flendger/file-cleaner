use io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn move_file(path: &Path, dest: &Path) -> io::Result<()> {
    if let Some(f_name) = path.file_name() {
        let dest_path = dest.join(f_name);

        fs::rename(path, dest_path)
    } else {
        Err(Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Path is not a file: {:?}", path),
        ))
    }
}
