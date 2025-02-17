use crate::remove_task::RemoveTask;
use crate::task_manager::TaskManager;
use crate::tasks::Task;

pub fn task_config() -> TaskManager {
    let mut tasks: Vec<Box<dyn Task>> = Vec::new();
    tasks.push(Box::from(RemoveTask::default()));

    TaskManager::new(tasks)
}
