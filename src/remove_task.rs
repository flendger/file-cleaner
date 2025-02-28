use std::io;
use crate::file_walker::walk_files;
use crate::file_remover::remove_file;
use crate::settings::Settings;
use crate::tasks::Task;

#[derive(Default)]
pub struct RemoveTask {}

impl Task for RemoveTask {
    fn run(&self, settings: &Settings) -> io::Result<()> {
        walk_files(settings, remove_file);
        Ok(())
    }

    fn task_type(&self) -> char {
        'r'
    }
}
