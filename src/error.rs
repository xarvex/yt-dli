use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::profile::PROFILE_DIRECTORY;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    FileContextError(#[from] FileContextError),

    #[error(transparent)]
    Dialoguer(#[from] dialoguer::Error),

    #[error(transparent)]
    IcuProviderData(#[from] icu_provider::DataError),

    #[error("{0}")]
    Simple(&'static str),

    #[error("no profiles found in '{}'", PROFILE_DIRECTORY.display())]
    MissingProfiles,
}

#[derive(Debug, Error)]
enum FileError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[error("while accessing '{}': {}", .context.display(), .source)]
pub struct FileContextError {
    source: FileError,
    context: PathBuf,
}

pub trait FileContextErrorExt {
    fn with_path<P: AsRef<Path>>(self, path: &P) -> FileContextError;
}

pub trait FileContextResultExt<T> {
    fn with_path<P: AsRef<Path>>(self, path: &P) -> std::result::Result<T, FileContextError>;
}

impl<E: Into<FileError>> FileContextErrorExt for E {
    fn with_path<P: AsRef<Path>>(self, path: &P) -> FileContextError {
        FileContextError {
            source: self.into(),
            context: path.as_ref().to_owned(),
        }
    }
}

impl<T, E: Into<FileError>> FileContextResultExt<T> for std::result::Result<T, E> {
    #[inline]
    fn with_path<P: AsRef<Path>>(self, path: &P) -> std::result::Result<T, FileContextError> {
        self.map_err(|e| e.with_path(path))
    }
}

impl From<Error> for clap::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::FileContextError { .. } => std::io::Error::other(error).into(),
            Error::Dialoguer(e) => match e {
                dialoguer::Error::IO(e) => e.into(),
            },
            Error::IcuProviderData(e) => std::io::Error::other(e).into(),
            Error::Simple(_) | Error::MissingProfiles => {
                clap::Error::raw(clap::error::ErrorKind::Io, error)
            }
        }
    }
}
