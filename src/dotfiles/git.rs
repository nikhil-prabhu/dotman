//! Module for working with Git.
use git2::{Error as GitError, ErrorCode, Repository};
use std::fs;
use std::path::PathBuf;

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
/// let src = "";
/// let dest = std::path::Path::new("");
/// let repo = dotfiles::clone(src, &dest, true).unwrap();
/// ```
///
/// ## Skip cloning if repository already exists locally
/// ```
/// use dotfiles;
///
/// let src = "";
/// let dest = std::path::Path::new("");
/// let repo = dotfiles::clone(src, &dest, false).unwrap();
/// ```
pub fn clone(src: &str, dest: &PathBuf, force: bool) -> Result<Repository, GitError> {
	let repo = Repository::clone(src, dest.clone());

	match repo {
		Ok(_) => repo,
		Err(e) => match e.code() {
			ErrorCode::Exists => {
				// If the local folder exists and we don't want to force the
				// clone operation, we just open the existing directory.
				if !force {
					return open(dest);
				}

				// If we want to force the clone operation when the destination
				// directory already exists, we remove the directory and then
				// call the clone operation again.
				fs::remove_dir_all(dest).unwrap();

				// NOTE: we specify `force=false` here to avoid any possibility
				// of an infinite recursion of the clone operation.
				return clone(src, dest, false);
			}
			// For any other error during the clone operation, we return the
			// error to the caller.
			_ => Err(e),
		},
	}
}
