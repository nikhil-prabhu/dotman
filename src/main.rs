use structopt::StructOpt;

/// > Help message goes here. <
#[derive(StructOpt)]
struct Flags {
    // Will be populated as functionality is added.
}

fn main() {
    // We won't use this variable yet. We just initialize the CLI app for now.
    let _flags = Flags::from_args();

    println!("A command line utility for managing *nix dotfiles; written in Rust.");
}
