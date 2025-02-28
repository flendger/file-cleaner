use std::{fs, io};
use std::io::ErrorKind;
use chrono::Local;
use crate::dest_dir_resolver::resolve_dest;
use crate::file_walker::walk_files;
use crate::file_mover::move_file;
use crate::settings::Settings;
use crate::tasks::Task;

#[derive(Default)]
pub struct MoveTask {}

impl Task for MoveTask {
    fn run(&self, settings: &Settings) -> io::Result<()> {
        let dest_path = &resolve_dest(&settings.dest_dir, &Local::now());

        if !dest_path.exists() {
            if let Err(error) = fs::create_dir_all(dest_path) {
                return Err(error);
            };
        }

        if !dest_path.is_dir() {
            return Err(io::Error::new(ErrorKind::NotADirectory, dest_path.to_str().unwrap()));
        }

        walk_files(settings, |p| {
            move_file(p, dest_path)
        });
        Ok(())
    }

    fn task_type(&self) -> char {
        'm'
    }
}
