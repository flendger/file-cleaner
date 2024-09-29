use std::io::Error;
use std::path::Path;

pub fn scan_dirs(folder: &String) -> Result<Vec<Box<Path>>, Error> {
    let path = Path::new(folder);

    let dirs = path.read_dir()?
        .flat_map(|dir| dir)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|path| path.into_boxed_path())
        .collect();

    Ok(dirs)
}