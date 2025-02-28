use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TaskError{
    desc: String,
}

impl TaskError {
    pub fn new(desc: &str) -> TaskError {
        TaskError{desc: String::from(desc)}
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.desc.as_str())
    }
}

impl Error for TaskError {
    fn description(&self) -> &str {
        &self.desc
    }
}