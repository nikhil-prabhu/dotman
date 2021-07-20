//! Module for working with dotfiles.

use git2::{Error as GitError, Repository};
use std::path::PathBuf;

pub mod git;

/// Clones a dotfiles repository onto the local system.
///
/// # Arguments
///
/// * `src` - The URL of the dotfiles repository.
/// * `dest` - The path to the local filesystem directory.
/// * `force` - Force the clone operation to overwrite an existing local directory.
pub fn clone(src: &str, dest: PathBuf, force: bool) -> Result<Repository, GitError> {
	git::clone(src, dest, force)
}
