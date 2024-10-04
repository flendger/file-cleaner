extern crate core;

use log::{error, info};
use crate::clean_manager::clean_files;
use crate::config_resolver::resolve_config;
use crate::logger_config::init_logger;
use crate::run_params::{get_args};
use crate::settings::Settings;
use crate::settings_reader::read_settings;

mod settings;
mod file_scanner;
mod dir_scanner;
mod file_remover;
mod dir_remover;
mod clean_manager;
mod settings_reader;
mod run_params;
mod config_resolver;
mod logger_config;

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

    for settings in settings_vec {
        clean_files(&settings);
    }
}
