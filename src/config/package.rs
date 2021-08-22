//! Module for performing package management tasks.
//! **WARNING: This module is highly experimental and should be used with caution.**
extern crate serde_json;

use crate::logger::Logger;
use packagekit::PackageKit;
use std::io::Write;

// TODO: add support for specifying package versions.
/// Installs a list of packages onto the system.
pub fn install<W>(args: &serde_json::Value, logger: &mut Logger<W>) -> Option<()>
where
    W: Write,
{
    if let Some(packages) = &args.as_array() {
        let pk = PackageKit::new();

        for pkg in packages.iter() {
            if let Some(s) = pkg.as_str() {
                // NOTE: Only the first matching package will be installed.
                // ? Maybe this isn't the best thing to do?
                let results = match pk.search_package(s) {
                    Ok(r) => r,
                    Err(e) => {
                        logger.error(&e.to_string());
                        return None;
                    }
                };

                logger.info(&format!("Installing package: {}", results[0].id()));
                match pk.install(&results[0]) {
                    Ok(_) => continue,
                    Err(e) => {
                        logger.error(&e.to_string());
                        return None;
                    }
                }
            } else {
                logger.error("Invalid package name in list of packages.");
                return None;
            }
        }
        logger.success("Done.");
        Some(())
    } else {
        logger.warn("Nothing to do.");
        Some(())
    }
}
