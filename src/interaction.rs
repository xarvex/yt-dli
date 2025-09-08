use std::sync::LazyLock;

use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};

use crate::{
    error::{Error, Result},
    profile::profiles,
    util::sort_dir_results,
};

static DIALOGUER_THEME: LazyLock<ColorfulTheme> = LazyLock::new(ColorfulTheme::default);

pub fn prompt_profiles() -> Result<Vec<String>> {
    let available_profiles = sort_dir_results(profiles()?)?;
    if available_profiles.is_empty() {
        return Err(Error::MissingProfiles);
    }

    if available_profiles.len() == 1 {
        let confirm = Confirm::with_theme(&*DIALOGUER_THEME)
            .with_prompt(format!(
                "Found '{}' as the only profile. Use it?",
                available_profiles[0]
            ))
            .interact()?;

        if confirm {
            Ok(available_profiles)
        } else {
            Err(Error::Simple("profile was rejected"))
        }
    } else {
        let profile_indices = MultiSelect::with_theme(&*DIALOGUER_THEME)
            .with_prompt("Select the profiles to use")
            .items(&available_profiles)
            .interact_opt()?
            .ok_or(Error::Simple("profile selection cancelled"))?;

        if profile_indices.is_empty() {
            Err(Error::Simple("no profiles were selected"))
        } else {
            // TODO: Find a way to avoid cloning and collecting.
            Ok(profile_indices
                .iter()
                .map(|i| available_profiles[*i].clone())
                .collect())
        }
    }
}

pub fn prompt_extra_args() -> Result<Vec<String>> {
    let raw_extra_args = dialoguer::Input::<String>::with_theme(&*DIALOGUER_THEME)
        .with_prompt("Enter arguments to pass to 'yt-dlp'")
        .interact_text()?;
    let extra_args = shlex::split(&raw_extra_args).ok_or(Error::Simple(
        "extra arguments meant for 'yt-dlp' could not be processed",
    ))?;

    if extra_args.is_empty() {
        Err(Error::Simple("no arguments for 'yt-dlp' were given"))
    } else {
        Ok(extra_args)
    }
}
