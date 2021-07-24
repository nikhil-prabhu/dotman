pub mod dotfiles;
pub mod logger;

use logger::{Logger, Target};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use terminal_size::{terminal_size, Height, Width};

const DEFAULT_TERM_WIDTH: u16 = 50; // 50 columns
const DOTMAN_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DOTMAN_LOGO: &str = "
       oooo             o8                                        
  ooooo888   ooooooo  o888oo oo ooo oooo    ooooooo   oo oooooo   
888    888 888     888 888    888 888 888   ooooo888   888   888  
888    888 888     888 888    888 888 888 888    888   888   888  
  88ooo888o  88ooo88    888o o888o888o888o 88ooo88 8o o888o o888o 
";

/// > Help message goes here. <
// TODO: Improve the flag names(?).
#[derive(StructOpt)]
struct Flags {
    #[structopt(short = "r", long = "repository")]
    repo: String,

    // TODO: special characters like `~` for $HOME currently don't work.
    #[structopt(short = "d", long = "dir")]
    dest: Option<PathBuf>,
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
fn banner(msg: &str, dec: Option<char>, width: Option<u16>) {
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

fn main() {
    println!("{}", DOTMAN_LOGO);
    // Disgusting way of centering the following text, but I have no alternative
    // at the moment.
    println!("                         VERSION: {}\n", DOTMAN_VERSION);

    // Parse command line flags and create a logger.
    let flags = Flags::from_args();
    let mut logger = Logger::new(Target::Stdout);

    // If the user did not specify a destination for the cloned dotfiles,
    // we use the current working directory.
    let dest = match flags.dest {
        Some(d) => d,
        None => {
            if let Ok(d) = env::current_dir() {
                d
            } else {
                // If current dir couldn't be determined, fail with fatal error.
                logger.fatal("Could not determine the current working directory.");

                // NOTE: The program terminates with the previous statement.
                // The following macro prevents the compiler from complaining
                // about match arms having incompatible types.
                unreachable!();
            }
        }
    };

    banner("TASK: Clone dotfiles.", None, None);
    // TODO: Improve all the following logging messages.
    logger.info(&format!("Cloning dotfiles to {}", &dest.display()));

    match dotfiles::clone(&flags.repo, &dest, true) {
        Ok(_) => logger.success(&format!(
            "Successfully cloned dotfiles to {}",
            dest.display(),
        )),
        // TODO: Print actual error message, rather than being a lazy twat.
        Err(_) => logger.fatal("An error occured while cloning dotfiles."),
    }
}
