use crate::settings::Settings;
use crate::task_mng_err::TaskError;
use crate::tasks::Task;
use std::collections::HashMap;
use std::error::Error;

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

    pub fn manage(&self, settings: &Settings) -> Result<(), Box<dyn Error>> {
        //resolve task handle
        if let Some(cur_task) = self.tasks_holder.get(&settings.cmd) {
            //run handler
            return if let Err(error) = cur_task.run(settings) {
                Err(Box::new(error))
            } else {
                Ok(())
            }
        };

        Err(Box::from(TaskError::new(
            format!("Task not found: {}", settings.cmd).as_str(),
        )))
    }
}
