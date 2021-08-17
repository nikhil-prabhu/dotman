extern crate serde_json;

use crate::logger::Logger;
use crate::shell;
use std::io::Write;

/// Runs a command in the shell.
pub fn run<W>(args: &serde_json::Value, logger: &mut Logger<W>)
where
    W: Write,
{
    if let Some(c) = &args.as_str() {
        logger.info(&format!("Running command: {}", c));
        shell::run(c);
        logger.info("Done.");
    } else {
        logger.warn("Nothing to do.");
    }
}
