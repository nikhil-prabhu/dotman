//!  Simple pretty logger
// TODO: Add emoji support to message labels(?)
extern crate chrono;

use ansi_term::{Color, Style};
use chrono::Local;
use std::{io, process};

/// Returns the current time in the format H:M:S (Hours, Minutes and Seconds)
fn get_fmt_time() -> String {
	let date = Local::now();
	format!("{}", date.format("%H:%M:%S"))
}

// TODO: Add support for using a file as a target.
/// The target for the logger to write to.
///
/// # Variants
///
/// * `Stdout` - The standard output stream `(std::io::Stdout)`
/// * `Stderr` - The standard error stream `(std::io::Stderr)`
pub enum Target {
	Stdout,
	Stderr,
}

/// The logger object.
///
/// # Fields
///
/// * `target` - The target for the logger to write to.
pub struct Logger {
	pub target: Box<dyn io::Write>,
}

impl Logger {
	/// Creates a new logger with the specified target.
	///
	/// # Arguments
	///
	/// * `target` - The target for the logger to write to.
	///
	/// # Examples
	///
	/// Creating a new logger that writes to stdout:
	/// ```
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
	/// ```
	///
	/// Creating a new logger that writes to stderr:
	/// ```
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stderr);
	/// ```
	pub fn new(target: Target) -> Self {
		if let Target::Stderr = target {
			return Self {
				target: Box::new(io::stderr()),
			};
		}

		Self {
			target: Box::new(io::stdout()),
		}
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
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
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
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
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
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
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
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
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
	/// use logger::{Logger, Target};
	///
	/// let mut logger = Logger::new(Target::Stdout);
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
