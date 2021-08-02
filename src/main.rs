pub mod config;
pub mod consts;
pub mod display;
pub mod dotfiles;
pub mod logger;
pub mod shell;

use logger::{Logger, Target};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() {
    display::print_logo();

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

    display::banner("TASK: Clone dotfiles.", None, None);
    // TODO: Improve all the following logging messages.
    logger.info(&format!("Cloning dotfiles to {}", &dest.display()));

    match dotfiles::clone(&flags.repo, &dest, true) {
        Ok(_) => logger.success(&format!(
            "Successfully cloned dotfiles to {}",
            dest.display(),
        )),
        // TODO: Some error messages are not very indicative of what actually went wrong.
        // Not yet sure of what I can do to fix this, considering the error message
        // string is provided directly by the git2 library.
        // ? Maybe a `match` on the ErrorKind?
        Err(e) => logger.fatal(&e.to_string()),
    }
}
