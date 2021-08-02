extern crate serde_json;

use crate::logger::Logger;
use crate::shell;

/// Runs a command in the shell.
pub fn run(args: &serde_json::Value, logger: &mut Logger) {
    if let Some(c) = &args.as_str() {
        logger.info(&format!("Running command: {}", c));
        shell::run(c);
        logger.info("Done.");
    } else {
        logger.warn("Nothing to do.");
    }
}
