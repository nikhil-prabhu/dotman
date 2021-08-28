//! Module for working with cron jobs.
//! **WARNING: This module is highly experimental and should be used with caution.**
extern crate cron_rs;
extern crate serde_json;
extern crate tempfile;

use crate::logger::Logger;
use crate::shell;

use cron_rs::Scheduler;
use std::env;
use std::fs::File;
use std::io::Write;

// TODO: add support for job description comments.
// TODO: test possible failure scenarios.
// TODO: handle `crontab` command failure.
// TODO: handle cron daemon not installed or not running.
// TODO: handle duplicate entries.
/// Adds a cron job for the current user.
pub fn add<W>(args: &serde_json::Value, logger: &mut Logger<W>) -> Option<()>
where
    W: Write,
{
    if let Some(cron) = &args.as_object() {
        logger.info("Adding cron job.");

        // Parse and validate specified cron intervals expression.
        let intervals = match cron.get("intervals") {
            Some(intervals) => match intervals.as_str() {
                Some(intervals) => intervals,
                None => {
                    logger.error("Intervals expression must be a string.");
                    return None;
                }
            },
            None => {
                logger.error("No intervals specified.");
                return None;
            }
        };

        // Parse and validate specified cron job.
        let job = match cron.get("job") {
            Some(job) => match job.as_str() {
                Some(job) => job,
                None => {
                    logger.error("Job must be a string.");
                    return None;
                }
            },
            None => {
                logger.error("No job specified.");
                return None;
            }
        };

        // Validate cron entry and install.
        // ? Could I possibly make the following code less nauseating to look at?
        if let Ok(_) = Scheduler::new(intervals) {
            // TODO: check if there's a better way to do this.
            let tmpdir = match env::var("TMPDIR") {
                Ok(tmpdir) => tmpdir,
                Err(_) => String::from("/tmp"),
            };
            let tmpdir = tempfile::tempdir_in(tmpdir).unwrap();
            let tmpfile = tmpdir.path().join("dotman.cron");

            // Get existing crontab entries and append to it.
            let crontab = shell::output("crontab", Some(&vec!["-l"]));
            let cron_out = String::from_utf8(crontab.unwrap().stdout).unwrap();
            let mut file = File::create(&tmpfile).unwrap();
            file.write_fmt(format_args!("{}\n", cron_out)).unwrap();
            file.write_fmt(format_args!("{} {}\n", intervals, job))
                .unwrap();

            // Install cron entry.
            shell::run(&format!("crontab -- {}", &tmpfile.display()));
            logger.success("Done.");

            Some(())
        } else {
            logger.error("Invalid cron interval expression.");

            None
        }
    } else {
        logger.warn("Nothing to do.");

        None
    }
}
