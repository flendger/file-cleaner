use std::path::Path;
use crate::dir_remover::remove_dir;
use crate::dir_scanner::scan_dirs;
use crate::file_remover::remove_file;
use crate::file_scanner::scan_files;
use crate::settings::Settings;

pub fn clean_files(settings: Vec<&Settings>) {
    for setting in settings {
        let depth = setting.depth;
        let initial_path = Path::new(&setting.folder);

        let mut processing_queue: Vec<Box<Path>> = Vec::new();
        processing_queue.push(Box::from(initial_path));

        while !processing_queue.is_empty() {
            let current_dir = processing_queue.pop().unwrap();
            let dir_ref = current_dir.as_ref();

            if !dir_ref.exists() {
                println!("Directory doesn't exist: {:?}", dir_ref);
                continue;
            }

            scan_files(dir_ref, depth)
                .unwrap_or_else(|_| {
                    println!("Failed to scan files in directory: {:?}", dir_ref);
                    vec![]
                })
                .iter()
                .for_each(|file| remove_file(&file)
                    .unwrap_or_else(|_| {
                        println!("Failed to remove file: {:?}", file);
                    }));

            let dirs = scan_dirs(dir_ref)
                .unwrap_or_else(|_| {
                    println!("Failed to scan directories: {:?}", dir_ref);
                    vec![]
                });
            if dirs.is_empty() {
                if initial_path != dir_ref {
                    remove_dir(dir_ref)
                        .unwrap_or_else(|_| {
                            println!("Failed to remove directory: {:?}", dir_ref);
                        });
                }

                continue;
            }

            processing_queue.push(current_dir);
            processing_queue.extend(dirs);
        }
    }
}