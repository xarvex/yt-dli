use std::ffi::{OsStr, OsString};

use clap::{
    Arg, Command, CommandFactory, Parser, Subcommand, ValueHint,
    builder::{ArgPredicate, TypedValueParser},
    error::{ContextKind, ContextValue, ErrorKind},
};
use clap_complete::{ArgValueCompleter, CompleteEnv, PathCompleter, Shell};

use crate::profile::{PROFILE_DIRECTORY, profile_exists, profile_path};

/// Thin wrapper around 'yt-dlp' allowing for profiles and quick interactive use.
///
///
/// Profiles are additions to one's 'yt-dlp' configuration that are meant for differentiating
/// circumstances. They can be combined, and can reference each other. They are added as an
/// additional config location to 'yt-dlp', so the syntax is the exact same. Profiles should be
/// placed in one's application config directory.
///
/// When invoked without a command, '-h/--help', or '-v/--version' flag, 'yt-dlp' will be invoked
/// according to the profile selection. If any parameters are missing, they will be interactively
/// asked for, though see '--no-interactive' for cancelling that behavior.
#[derive(Parser)]
#[command(version, author, about)]
pub struct Cli {
    /// Allow use of interactive prompts.
    ///
    /// This is only invoked when necessary to get incomplete information. This is the default
    /// behavior, which can be negated with '--no_interactive'. Specifying this flag after
    /// disabling will re-enable.
    #[arg(
        short,
        long,
        default_value_if("no_interactive", ArgPredicate::IsPresent, "false"),
        default_value_t = true,
        overrides_with = "no_interactive"
    )]
    pub interactive: bool,

    /// Disallow use of interactive prompts.
    ///
    /// This will prevent user-facing prompts when information is incomplete. This means that in
    /// place of these prompts, an error will be emitted. This overrides the default behavior that
    /// '--interactive' provides.
    #[arg(short = 'I', long, overrides_with = "interactive")]
    no_interactive: bool,

    #[command(subcommand)]
    pub subcommand: Option<CliSubcommand>,

    /// Specify the profile(s) to use.
    ///
    /// See this command's about for what profiles are. This option can be given one profile or a
    /// list of profiles, lists having each profile be separated by a comma (','). Additionally,
    /// this option can be specified multiple times to add more profile(s) on.
    #[arg(
        id = "profile",
        short,
        long,
        add = ArgValueCompleter::new(PathCompleter::file().current_dir(PROFILE_DIRECTORY.as_path())),
        required_if_eq("no_interactive", "true"),
        value_delimiter = ',',
        value_hint = ValueHint::Other,
        value_name = "PROFILES",
        value_parser = ProfileValueParser,
        visible_alias = "profiles"
    )]
    pub profiles: Option<Vec<OsString>>,

    /// Arguments passed down to 'yt-dlp'.
    ///
    /// These are given in addition to profiles, typically used to specify video URLs to download,
    /// or any other arguments that don't belong in a config or profile entry.
    #[arg(name = "yt-dlp OPTIONS", trailing_var_arg = true, value_hint = ValueHint::Other)]
    pub extra_args: Option<Vec<OsString>>,
}

impl Cli {
    pub const COMPLETION_VAR: &str = "YTDLI_COMPLETE";

    pub fn completion_factory() {
        CompleteEnv::with_factory(Self::command)
            .var(Self::COMPLETION_VAR)
            .complete();
    }
}

#[derive(Subcommand)]
#[command(subcommand_negates_reqs = true)]
pub enum CliSubcommand {
    /// Generates a shell completion script.
    ///
    /// Either source this during a session, or pipe to a file that is sourced by your shell init.
    /// Completion may already be provided if this program came from your system's package manager
    /// (not cargo). Package maintainers should ideally provide this in the build process.
    Completions {
        /// Shell to generate completions for.
        ///
        /// Specify the shell you are currently using or targeting.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Lists all of the profiles found that can be used.
    ///
    /// This list is sorted according to locale and does take into account numerical ordering.
    ListProfiles,
}

#[derive(Clone)]
struct ProfileValueParser;

impl clap::builder::TypedValueParser for ProfileValueParser {
    type Value = OsString;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> clap::error::Result<Self::Value> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }
    fn parse(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: OsString,
    ) -> clap::error::Result<Self::Value> {
        let value = clap::builder::OsStringValueParser::new().parse(cmd, arg, value)?;

        match profile_exists(&value) {
            Ok(true) => Ok(value),
            Ok(false) => {
                let styles = cmd.get_styles();
                let literal = styles.get_literal();

                let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
                if let Some(arg) = arg {
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(arg.to_string()),
                    );
                }
                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(value.to_string_lossy().into_owned()),
                );
                err.insert(
                    ContextKind::Suggested,
                    ContextValue::StyledStrs(vec![
                        format!(
                            "ensure the profile exists at '{literal}{}{literal:#}'",
                            profile_path(&value).display()
                        )
                        .into(),
                    ]),
                );

                Err(err)
            }
            Err(e) => Err(clap::Error::from(e).with_cmd(cmd)),
        }
    }
}
