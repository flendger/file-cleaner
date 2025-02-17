use crate::settings::Settings;
use crate::tasks::Task;
use std::collections::HashMap;

pub struct TaskManager {
    tasks_holder: HashMap<char, Box<dyn Task>>,
}

impl TaskManager {
    pub fn new(tasks: Vec<Box<dyn Task>>) -> Self {
        let mut task_holder: HashMap<char, Box<dyn Task>> = HashMap::new();
        for task in tasks {
            task_holder.insert(task.task_type(), task);
        }

        Self {
            tasks_holder: task_holder,
        }
    }

    pub fn manage(&self, settings: &Settings) {
        //resolve task handle
        if let Some(cur_task) = self.tasks_holder.get(&settings.cmd) {
            //run handler
            cur_task.run(settings);
        };

        // TODO: 15.02.2025 ekiru --> handle if no tasks found
    }
}
