//! Module for performing script related tasks.
extern crate serde_json;

use crate::logger::Logger;
use crate::shell;
use std::io::Write;
use std::path::PathBuf;

/// Runs a script.
pub fn run<W>(args: &serde_json::Value, logger: &mut Logger<W>) -> Option<()>
where
    W: Write,
{
    if let Some(s) = &args.as_str() {
        logger.info(&format!("Running script: {}", s));
        if shell::run_script(&PathBuf::from(s)) {
            logger.success("Done");
            Some(())
        } else {
            logger.error("An error occured while running the script.");
            None
        }
    } else {
        logger.warn("Nothing to do.");
        Some(())
    }
}
