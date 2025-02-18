use crate::file_walker::walk_files;
use crate::file_mover::move_file;
use crate::settings::Settings;
use crate::tasks::Task;

#[derive(Default)]
pub struct MoveTask {}

impl Task for MoveTask {
    fn run(&self, settings: &Settings) {
        walk_files(settings, |p| {
            move_file(p, settings.dest_dir.as_str())
        });
    }

    fn task_type(&self) -> char {
        'm'
    }
}
