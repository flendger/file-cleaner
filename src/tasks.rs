use std::io;
use crate::settings::Settings;

pub trait Task {
    // TODO: 15.02.2025 ekiru --> use trait type ???

    fn run(&self, settings: &Settings) -> io::Result<()>;
    fn task_type(&self) -> char;
}
