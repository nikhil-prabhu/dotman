//!  Simple pretty logger
// TODO: Add emoji support to message labels(?)
extern crate chrono;

use ansi_term::{Color, Style};
use chrono::Local;
use std::{
    io::{self, Stdout, Write},
    process,
};

/// Returns the current time in the format H:M:S (Hours, Minutes and Seconds)
fn get_fmt_time() -> String {
    let date = Local::now();
    format!("{}", date.format("%H:%M:%S"))
}

/// The logger object.
///
/// # Fields
///
/// * `target` - The target for the logger to write to.
pub struct Logger<W>
where
    W: Write,
{
    target: W,
}

impl<W> Logger<W>
where
    W: Write,
{
    /// Creates a new logger that writes to the specified target.
    ///
    /// # Arguments
    ///
    /// * `target` - The target for the logger to write to.
    ///
    /// # Examples
    ///
    /// Creating a new logger that writes to stderr:
    /// ```
    /// use logger::Logger;
    /// use std::io::prelude::*;
    ///
    /// let mut logger = Logger::from(Box::new(std::io::stderr()));
    /// ```
    ///
    /// Creating a new logger that writes to a file:
    /// ```
    /// use logger::Logger;
    /// use std::io::prelude::*;
    ///
    /// let out = std::fs::File::create("/home/johndoe/log.txt").unwrap();
    /// let mut logger = Logger::from(out);
    /// ```
    pub fn from(target: W) -> Self {
        Self { target }
    }

    /// Writes a message to the target with the label `INFO` and the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// logger.info("This is an info message.");
    /// ```
    pub fn info(&mut self, msg: &str) {
        writeln!(
            self.target,
            "{} [{}]: {}",
            Color::Blue.bold().paint("INFO"),
            Style::new().bold().paint(get_fmt_time()),
            msg,
        )
        .unwrap();
    }

    /// Writes a message to the target with the label `SUCCESS` and the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// logger.success("This is a success message.");
    /// ```
    pub fn success(&mut self, msg: &str) {
        writeln!(
            self.target,
            "{} [{}]: {}",
            Color::Green.bold().paint("SUCCESS"),
            Style::new().bold().paint(get_fmt_time()),
            msg,
        )
        .unwrap();
    }

    /// Writes a message to the target with the label `WARNING` and the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// logger.warn("This is a warning message.");
    /// ```
    pub fn warn(&mut self, msg: &str) {
        writeln!(
            self.target,
            "{} [{}]: {}",
            Color::Yellow.bold().paint("WARNING"),
            Style::new().bold().paint(get_fmt_time()),
            msg,
        )
        .unwrap();
    }

    /// Writes a message to the target with the label `ERROR` and the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// logger.error("This is an error message.");
    /// ```
    pub fn error(&mut self, msg: &str) {
        writeln!(
            self.target,
            "{} [{}]: {}",
            Color::Red.bold().paint("ERROR"),
            Style::new().bold().paint(get_fmt_time()),
            msg,
        )
        .unwrap();
    }

    /// Writes a message to the target with the label `FATAL` and the current timestamp.
    /// Also terminates the program with exit code 1.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// logger.fatal("This is a fatal error message.");
    /// ```
    pub fn fatal(&mut self, msg: &str) {
        writeln!(
            self.target,
            "{} [{}]: {}",
            Color::Red.bold().paint("FATAL"),
            Style::new().bold().paint(get_fmt_time()),
            msg,
        )
        .unwrap();

        process::exit(1);
    }
}

impl Logger<Stdout> {
    /// Creates a new logger that writes to stdout.
    ///
    /// # Examples
    ///
    /// Creating a new logger that writes to stdout:
    /// ```
    /// use logger::Logger;
    ///
    /// let mut logger = Logger::new();
    /// ```
    pub fn new() -> Self {
        Self::from(io::stdout())
    }
}
