//! Console display and pretty printing utilities.
use crate::consts::*;
use terminal_size::{terminal_size, Height, Width};

/// Prints the dotman logo along with the version.
pub fn print_logo() {
	println!("{}", DOTMAN_LOGO);

	// Disgusting way of centering the following text, but I have no alternative
	// at the moment.
	println!("                         VERSION: {}\n", DOTMAN_VERSION);
}

// ? Implementation could probably be improved.
/// Prints a banner on the console with the specified message.
///
/// The function allows the caller to specify the decorator to use and the
/// desired width of the banner. If neither of these are specified OR if the
/// console width cannot be determined, default values are used instead.
///
/// # Arguments
///
/// * `msg` - The message to display.
/// * `dec` - The character to use as a decorator.
/// * `width` - The width of the banner.
pub fn banner(msg: &str, dec: Option<char>, width: Option<u16>) {
	let size = terminal_size();
	let dec = match dec {
		Some(d) => d,
		None => '*',
	};
	let width: u16 = match width {
		Some(w) => w,
		None => {
			if let Some((Width(w), Height(_))) = size {
				w
			} else {
				DEFAULT_TERM_WIDTH
			}
		}
	};

	// Generate the banner lines from the separator using the width.
	let mut iter = 0;
	let mut line = String::new();

	while iter < width {
		line.push(dec);
		iter += 1;
	}

	// Print the banner.
	println!("{}", line);
	println!("{}", msg);
	println!("{}", line);
	println!();
}
