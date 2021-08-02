//! Module for performing package related tasks.
extern crate serde_json;

use crate::logger::Logger;

/// Installs a list of packages onto the system.
pub fn install(_args: &serde_json::Value, logger: &mut Logger) {
    // TODO: implement.
    logger.warn("Nothing to do.");
}
