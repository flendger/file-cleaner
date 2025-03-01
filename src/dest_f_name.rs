use chrono::Local;
use std::path::Path;

pub fn resolve_dest_f_name(dest: &Path) -> Option<String> {
    if let Some(f_name) = dest.file_stem() {
        let timestamp = Local::now().timestamp();

        let mut dest_f_name = String::from(f_name.to_str().unwrap());
        dest_f_name.push('_');
        dest_f_name.push_str(timestamp.to_string().as_str());

        if let Some(extension) = dest.extension() {
            dest_f_name.push('.');
            dest_f_name.push_str(extension.to_str().unwrap());
        }

        Some(dest_f_name)
    } else {
        None
    }
}
