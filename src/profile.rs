use std::{
    ffi::{OsStr, OsString},
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{
    error::{Error, Result},
    util::DIRECTORIES,
};

pub static PROFILE_DIRECTORY: LazyLock<&Path> = LazyLock::new(|| DIRECTORIES.config_dir());

pub fn profile_path<S: AsRef<OsStr>>(profile: S) -> PathBuf {
    PROFILE_DIRECTORY.join(profile.as_ref())
}

pub fn profiles() -> Result<ReadDir> {
    match fs::read_dir(*PROFILE_DIRECTORY) {
        Ok(r) => Ok(r),
        Err(e) => Err(match e.kind() {
            io::ErrorKind::NotFound => Error::MissingProfiles,
            _ => (e, PROFILE_DIRECTORY.to_path_buf()).into(),
        }),
    }
}

pub fn profile_exists(profile: &OsString) -> Result<bool> {
    let path = profile_path(profile);
    fs::exists(&path).map_err(|e| (e, path).into())
}
