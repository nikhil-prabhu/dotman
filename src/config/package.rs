//! Module for performing package related tasks.
extern crate serde_json;

use crate::logger::Logger;
use std::io::Write;

/// Installs a list of packages onto the system.
pub fn install<W>(_args: &serde_json::Value, logger: &mut Logger<W>)
where
    W: Write,
{
    // TODO: implement.
    logger.warn("Nothing to do.");
}
