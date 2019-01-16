//! Windows-specific implementations.

use crate::unit::{AddMode, Symlink};
use failure::{bail, Error};
use std::borrow::Cow;
use std::env::consts;
use std::path::{Path, PathBuf};

/// Convert into an executable path.
pub fn exe_path(mut path: PathBuf) -> PathBuf {
    if path.extension() == Some(consts::EXE_EXTENSION.as_ref()) {
        return path;
    }

    path.set_extension(consts::EXE_EXTENSION);
    path
}

/// Convert the given command into a path.
///
/// This adds the platform-specific extension for Windows.
pub fn command<'a>(base: &'a str) -> Cow<'a, Path> {
    Cow::from(exe_path(PathBuf::from(base)))
}

/// Add the given modes (on top of the existing ones).
pub fn add_mode(mode: &AddMode) -> Result<(), Error> {
    if mode.is_executable() {
        // NB: windows files are executable if they have the .exe extension.
        if mode.path.extension() != Some(consts::EXE_EXTENSION.as_ref()) {
            bail!("non-exe files cannot be executable");
        }
    }

    Ok(())
}

/// Create a symlink.
pub fn create_symlink(symlink: &Symlink) -> Result<(), Error> {
    use std::fs;
    use std::os::windows::fs::{symlink_dir, symlink_file};

    let Symlink {
        remove,
        ref path,
        ref link,
    } = *symlink;

    if remove {
        log::info!("re-linking {} to {}", path.display(), link.display());
        fs::remove_file(&path)?;
    } else {
        log::info!("linking {} to {}", path.display(), link.display());
    }

    if path.is_file() {
        symlink_file(path, path.join(&link))?;
        return Ok(());
    }

    if path.is_dir() {
        symlink_dir(path, path.join(&link))?;
        return Ok(());
    }

    bail!(
        "cannot symlink `{}`: not a file or directory",
        path.display()
    );
}
