use std::{env, process::ExitCode};

use clap::{CommandFactory, Parser};

use crate::{
    cli::{Cli, CliSubcommand},
    error::Result,
    exec::ytdlp,
    interaction::{prompt_extra_args, prompt_profiles},
    profile::profiles,
    util::{IntoOsStringIter, sort_dir_results},
};

mod cli;
mod error;
mod exec;
mod interaction;
mod profile;
mod util;

fn main() -> ExitCode {
    Cli::completion_factory();

    let mut cmd = Cli::command();
    let cli = Cli::parse();

    run(cli).unwrap_or_else(|e| clap::Error::from(e).format(&mut cmd).exit())
}

fn run(cli: Cli) -> Result<ExitCode> {
    if let Some(subcommand) = cli.subcommand {
        match subcommand {
            CliSubcommand::Completions { shell } => {
                // Program is single-threaded, so this operation is safe. Must do this as
                // `clap_complete` doesn't have a way to use the completion generation directly.
                unsafe {
                    env::set_var(Cli::COMPLETION_VAR, shell.to_string());
                }
                Cli::completion_factory();
            }
            CliSubcommand::ListProfiles => {
                let profiles = sort_dir_results(profiles()?)?;
                println!("{}", profiles.join("\n"));
            }
        }

        Ok(ExitCode::default())
    } else {
        let profiles: IntoOsStringIter = if let Some(profiles) = cli.profiles {
            profiles.into()
        } else if cli.interactive {
            prompt_profiles()?.into()
        } else {
            unreachable!("'profiles' should be required by clap")
        };

        let extra_args: IntoOsStringIter = if let Some(extra_args) = cli.extra_args {
            extra_args.into()
        } else if cli.interactive {
            prompt_extra_args()?.into()
        } else {
            unreachable!("'extra_args' should be required by clap")
        };

        ytdlp(Some(profiles), Some(extra_args))
    }
}
