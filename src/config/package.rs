//! Module for performing package management tasks.
//! **WARNING: This module is highly experimental and should be used with caution.**
extern crate serde_json;

use crate::logger::Logger;
use packagekit::PackageKit;
use std::io::Write;

// TODO: add support for specifying package versions.
/// Installs a list of packages onto the system.
pub fn install<W>(args: &serde_json::Value, logger: &mut Logger<W>)
where
    W: Write,
{
    if let Some(packages) = &args.as_array() {
        let pk = PackageKit::new();

        // TODO: For now we just unwrap on error. Proper error handling should be added.
        for pkg in packages.iter() {
            if let Some(s) = pkg.as_str() {
                // NOTE: Only the first matching package will be installed.
                // ? Maybe this isn't the best thing to do?
                let results = pk.search_package(s).unwrap();

                logger.info(&format!("Installing package: {}", results[0].id()));
                pk.install(&results[0]).unwrap();
            } else {
                // ? Should probably handle invalid args in a better way.
                logger.fatal("Invalid package name in list of packages.");
            }
        }
        logger.success("Done.");
    } else {
        logger.warn("Nothing to do.");
    }
}
