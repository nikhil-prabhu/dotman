//! Module for performing script related tasks.
extern crate serde_json;

use crate::logger::Logger;
use crate::shell;
use std::path::PathBuf;

/// Runs a script.
pub fn run(args: &serde_json::Value, logger: &mut Logger) {
    if let Some(s) = &args.as_str() {
        logger.info(&format!("Running script: {}", s));
        if shell::run_script(&PathBuf::from(s)) {
            logger.success("Done");
        } else {
            logger.error("An error occured while running the script.");
        }
    } else {
        logger.warn("Nothing to do.")
    }
}
