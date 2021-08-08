//! Module for parsing dotman config files.
extern crate serde_json;

use crate::display;
use crate::logger::Logger;

use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fs, io::BufReader, path::PathBuf};

// Available configuration modules.
mod command;
mod package;
mod script;

/// Represents a module's callback function.
type ModuleCallback = fn(&Value, &mut Logger);

/// Represents a dotman task to perform.
///
/// # Fields
///
/// * `name` - The name of the task (will be displayed in the task banner).
/// * `module` - The module to use for running the task.
/// * `args` - The arguments to be passed to the module.
#[derive(Clone, Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub module: String,
    pub args: Value,
}

/// Represents a dotman configuration.
///
/// # Fields
///
/// * `tasks` - The list (vector) of tasks that dotman has to perform.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>,
}

/// Parses and returns a JSON configuration.
///
/// # Arguments
///
/// * `file` - The path to the configuration file.
///
/// # Examples
///
/// ```
/// let file = std::path::PathBuf::from("/home/johndoe/config.json");
/// let config = config::parse(&file);
/// println!("{:#?}", config);
/// ```
pub fn parse(file: &PathBuf) -> Config {
    let file = fs::File::open(&file).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();

    config
}

/// Runs the list of tasks defined in a configuration.
///
/// # Arguments
///
/// * `config` - The configuration containing the tasks to run.
/// * `logger` - The logger to write to.
///
/// # Examples
///
/// ```
/// let file = std::path::PathBuf::from("/home/johndoe/config.json");
/// let mut logger = logger::Logger::new();
/// let config = config::parse(&file);
///
/// config::run_tasks(&config, &mut logger);
/// ```
pub fn run_tasks(config: &Config, logger: &mut Logger) {
    let mut module_dispatcher: HashMap<String, ModuleCallback> = HashMap::new();
    let tasks: &Vec<Task> = &config.tasks;

    // We use a hashmap to map each module with its callback function.
    module_dispatcher.insert(String::from("command"), command::run);
    module_dispatcher.insert(String::from("package"), package::install);
    module_dispatcher.insert(String::from("script"), script::run);

    // Iterate through and run each task.
    for task in tasks.iter() {
        display::banner(&format!("TASK: {}", &task.name), None, None);
        module_dispatcher[&task.module](&task.args, logger);
        println!();
    }
}
