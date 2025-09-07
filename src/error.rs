use thiserror::Error;

use crate::profile::PROFILE_DIRECTORY;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Dialoguer(#[from] dialoguer::Error),

    #[error(transparent)]
    IcuProviderData(#[from] icu_provider::DataError),

    #[error("{0}")]
    Simple(&'static str),

    #[error("no profiles found in '{}'", PROFILE_DIRECTORY.display())]
    MissingProfiles,
}

impl From<Error> for clap::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(e) => e.into(),
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
