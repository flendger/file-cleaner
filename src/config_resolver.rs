use std::env::current_dir;
use std::path::Path;

const D_FILE_NAME: &str = "app.json";

pub fn resolve_config(c_path: &str) -> Box<Path> {
    if c_path.is_empty() {
        default_config()
    } else {
        Path::new(c_path)
            .to_path_buf()
            .into_boxed_path()
    }
}

fn default_config() -> Box<Path> {
    let c_dir = current_dir().unwrap();

    c_dir
        .join(D_FILE_NAME)
        .into_boxed_path()
}