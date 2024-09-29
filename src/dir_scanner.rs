use std::io::Error;
use std::path::Path;

pub fn scan_dirs(folder: &Path) -> Result<Vec<Box<Path>>, Error> {
    let dirs = folder.read_dir()?
        .flat_map(|dir| dir)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|path| path.into_boxed_path())
        .collect();

    Ok(dirs)
}