extern crate core;

use crate::config_resolver::resolve_config;
use crate::logger_config::init_logger;
use crate::run_params::get_args;
use crate::settings::Settings;
use crate::settings_reader::read_settings;
use crate::tsk_mng_config::task_config;
use log::{error, info};

mod settings;
mod file_scanner;
mod dir_scanner;
mod file_remover;
mod dir_remover;
mod file_walker;
mod settings_reader;
mod run_params;
mod config_resolver;
mod logger_config;
mod task_manager;
mod tasks;
mod remove_task;
mod tsk_mng_config;
mod move_task;
mod file_mover;
mod dest_dir_resolver;
mod task_mng_err;

fn main() {
    let args = get_args();
    let path = resolve_config(args.config.as_str());
    let path_str = path.to_str().unwrap();

    init_logger(args.log.as_str());

    info!("Config: {:?}", path_str);

    let mut log_info: &str = args.log.as_str();
    if log_info.is_empty() {
        log_info = "console";
    }
    info!("Logger: {:?}", log_info);

    let settings_vec: Vec<Settings> = read_settings(path_str)
        .unwrap_or_else(|e| {
            error!("Couldn't read settings cause: {:?}", e.to_string());
            vec![]
        });

    let task_manager = task_config();

    for settings in settings_vec {
        if let Err(error) = task_manager.manage(&settings) {
            error!("{}", error);
        };
    }
}
