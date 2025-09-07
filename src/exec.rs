use std::{
    ffi::OsStr,
    process::{self, ExitCode},
};

use crate::{error::Result, profile::profile_path};

pub fn ytdlp<PI, PS, EI, ES>(profiles: Option<PI>, extra_args: Option<EI>) -> Result<ExitCode>
where
    PI: IntoIterator<Item = PS>,
    PS: AsRef<OsStr>,
    EI: IntoIterator<Item = ES>,
    ES: AsRef<OsStr>,
{
    let mut ytdlp = process::Command::new("yt-dlp");
    if let Some(profiles) = profiles {
        for profile in profiles {
            ytdlp.arg("--config-location").arg(profile_path(profile));
        }
    }
    if let Some(extra_args) = extra_args {
        ytdlp.args(extra_args);
    }

    Ok(ytdlp
        .status()?
        .code()
        .map(|c| ExitCode::from(c as u8))
        .unwrap_or(ExitCode::FAILURE))
}
