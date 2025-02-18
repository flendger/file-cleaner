use crate::file_walker::walk_files;
use crate::file_remover::remove_file;
use crate::settings::Settings;
use crate::tasks::Task;

#[derive(Default)]
pub struct RemoveTask {}

impl Task for RemoveTask {
    fn run(&self, settings: &Settings) {
        walk_files(settings, remove_file);
    }

    fn task_type(&self) -> char {
        'r'
    }
}
