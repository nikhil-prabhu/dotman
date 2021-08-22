extern crate serde_json;

use crate::logger::Logger;
use crate::shell;
use std::io::Write;

// TODO: Handle possible command execution errors.
/// Runs a command in the shell.
pub fn run<W>(args: &serde_json::Value, logger: &mut Logger<W>) -> Option<()>
where
    W: Write,
{
    if let Some(c) = &args.as_str() {
        logger.info(&format!("Running command: {}", c));
        shell::run(c);
        logger.info("Done.");
        Some(())
    } else {
        logger.warn("Nothing to do.");
        Some(())
    }
}
