//! Module for working with shell operations.
use std::{
    fs::metadata,
    io,
    os::unix::fs::MetadataExt,
    path::PathBuf,
    process::{Child, Command, ExitStatus, Output},
};

/// Returns a process::Command object.
///
/// # Arguments
///
/// * `cmd` - The command.
/// * `args` - And optional vector of arguments for the command.
fn command(cmd: &str, args: Option<&Vec<&str>>) -> Command {
    let mut cmd = Command::new(cmd);

    if let Some(args) = args {
        cmd.args(args);
    }

    cmd
}

// TODO: add more examples.
/// Spawns a child process and returns a handle to it.
///
/// # Arguments
///
/// * `cmd` - The command to run.
/// * `args` - An optional vector of arguments for the command.
///
/// # Examples
///
/// Run a command and panic on error:
/// ```
/// shell::spawn("echo", Some(&vec!["Hello", "world"])).unwrap();
/// ```
pub fn spawn(cmd: &str, args: Option<&Vec<&str>>) -> io::Result<Child> {
    command(cmd, args).spawn()
}

// TODO: add more examples.
/// Runs a command in a child process and returns the collected output after waiting
/// for the process to finish.
///
/// The process' stdout and stderr are captured in the output.
///
/// # Arguments
///
/// * `cmd` - The command to run.
/// * `args` - An optional vector of arguments for the command.
///
/// # Examples
///
/// Run a command and panic on error:
/// ```
/// shell::output("echo", Some(&vec!["Hello", "world"])).unwrap();
/// ```
pub fn output(cmd: &str, args: Option<&Vec<&str>>) -> io::Result<Output> {
    command(cmd, args).output()
}

// TODO: add more examples.
/// Runs a command in a child process and waits for it to finish and collects
/// its status.
///
/// # Arguments
///
/// * `cmd` - The command to run.
/// * `args` - An optional vector of arguments for the command.
///
/// # Examples
///
/// Run a command and panic on error:
/// ```
/// shell::status("echo", Some(&vec!["Hello", "world"])).unwrap();
/// ```
pub fn status(cmd: &str, args: Option<&Vec<&str>>) -> io::Result<ExitStatus> {
    command(cmd, args).status()
}

/// Runs a command as a child process in the shell.
///
/// This is a convenience function that's used to just run a command without
/// having to care about the returned value (i.e. the returned `ExitStatus` is
/// discarded). To have greater control, use either the `spawn()`, `output()`
/// or the `status()` function instead.
///
/// # Arguments
///
/// * `cmd` - The command to run along with any required arguments.
///
/// # Examples
///
/// Run a command in the shell:
/// ```
/// shell::run("echo Hello world");
/// ```
pub fn run(cmd: &str) {
    // Get list of arguments by splitting the command string on whitespace.
    let args = cmd.split(char::is_whitespace).collect::<Vec<&str>>();
    // Separate the command from the arguments.
    let cmd = args[0];
    let args = Vec::from(&args[1..args.len()]);

    // If the arguments vector has only one element, then it's safe to assume
    // that the command does not take any arguments.
    if args.len() == 1 {
        let _ = status(cmd, None);
    }

    let _ = status(cmd, Some(&args));
}

// TODO: add option to write script output to a stream/file.
// TODO: add option to specify script interpreter to use.
/// Executes a script and returns a boolean indicating if the script ran successfully
/// or failed (`true` indicates success and `false` indicates failure).
///
/// Currently, it is mandatory for the script to contain a shebang specifying
/// the interpreter to use to run the script. This may change in the future.
///
/// # Arguments
///
/// * `path` - The path to the script to execute.
///
/// # Examples
///
/// Running a script and printing a message based on success or failure:
/// ```
/// let script = std::path::PathBuf::from("/usr/bin/hello.sh");
///
/// if shell::run(&script) {
/// 	println!("Script successfully executed.");
/// } else {
/// 	println!("Script failed to execute.");
/// }
/// ```
pub fn run_script(path: &PathBuf) -> bool {
    if let Ok(_) = output(path.to_str().unwrap(), None) {
        return true;
    }

    false
}

/// Returns the effective user ID of current user.
fn geteuid() -> u32 {
    // NOTE: We assume that the `/proc/self` file always exists, which is why
    // we just unwrap the Result.
    metadata("/proc/self").map(|m| m.uid()).unwrap()
}

/// Returns a boolean indicating whether or not the current user is root.
pub fn is_root() -> bool {
    geteuid() == 0
}
