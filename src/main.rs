pub mod dotfiles;
pub mod logger;

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
                // If current dir couldn't be determined, return an empty PathBuf.
                PathBuf::new()
            }
        }
    };

    if !dest.exists() {
        logger.fatal("Could not determine the current working directory.");
    }

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
