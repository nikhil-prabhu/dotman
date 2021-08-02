//! Module for working with Git.
// TODO: Add authentication support for cloning private repos.
use fs_extra::dir::{move_dir, CopyOptions};
use git2::{Error as GitError, Repository};
use std::{fs, path::PathBuf};

/// Opens an existing repository on the filesystem.
///
/// # Arguments
///
/// * `src` - The path to the repository on the filesystem.
fn open(src: &PathBuf) -> Result<Repository, GitError> {
    Repository::open(src)
}

/// Clones a repository from a remote location to the local filesystem.
///
/// # Arguments
///
/// * `src` - The remote URL.
/// * `dest` - The local destination directory.
/// * `force` - Whether the clone operation should overwrite an existing destination.
///
/// # Examples
///
/// ## Forcefully cloning when destination exists
/// ```
/// use dotfiles;
///
/// let src = "https://github.com/john-doe/hello-world";
/// let dest = std::path::PathBuf::from("/home/johndoe/hello-world");
/// let repo = dotfiles::clone(src, &dest, true).unwrap();
/// ```
///
/// ## Skip cloning if repository already exists locally
/// ```
/// use dotfiles;
///
/// let src = "https://github.com/john-doe/hello-world";
/// let dest = std::path::PathBuf::from("/home/johndoe/hello-world");
/// let repo = dotfiles::clone(src, &dest, false).unwrap();
/// ```
pub fn clone(src: &str, dest: &PathBuf, force: bool) -> Result<Repository, GitError> {
    // Check if destination directory already exists locally.
    if dest.exists() {
        // If `force` is set to `false`, we skip the clone operation and just
        // open the local directory.
        if !force {
            return open(&dest);
        }

        // If `force` is set to `true`, we clone the repository to the UNIX
        // `/tmp` directory (to make sure that the repository actually exists
        // in the specified source) and then move the cloned directory to the
        // specified destination (this overwrites the existing directory).
        let mut temp_dir = std::env::temp_dir();
        if let Some(dir) = &dest.file_name() {
            temp_dir.push(dir);
        }

        let repo = Repository::clone(src, &temp_dir);

        if let Ok(_) = repo {
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.content_only = true;

            fs::remove_dir_all(&dest).unwrap();
            move_dir(&temp_dir, &dest, &options).unwrap();

            return open(&dest);
        } else {
            return repo;
        }
    }

    // If the destination directory does not exist locally, we can safely clone
    // the repository with no further checks required.
    Repository::clone(src, &dest)
}
