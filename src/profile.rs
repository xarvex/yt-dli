use std::{
    ffi::{OsStr, OsString},
    fs::{self, ReadDir},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{error::Result, util::DIRECTORIES};

pub static PROFILE_DIRECTORY: LazyLock<&Path> = LazyLock::new(|| DIRECTORIES.config_dir());

pub fn profile_path<S: AsRef<OsStr>>(profile: S) -> PathBuf {
    PROFILE_DIRECTORY.join(profile.as_ref())
}

pub fn profiles() -> Result<ReadDir> {
    Ok(fs::read_dir(*PROFILE_DIRECTORY)?)
}

pub fn profile_exists(profile: &OsString) -> Result<bool> {
    Ok(fs::exists(profile_path(profile))?)
}
