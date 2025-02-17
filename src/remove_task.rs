use crate::clean_manager::clean_files;
use crate::settings::Settings;
use crate::tasks::Task;

pub struct RemoveTask {}

impl Task for RemoveTask {
    fn run(&self, settings: &Settings) {
        clean_files(settings);
    }

    fn task_type(&self) -> char {
        'r'
    }
}
