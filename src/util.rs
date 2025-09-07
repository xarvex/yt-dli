use std::{convert::From, ffi::OsString, fs::DirEntry, sync::LazyLock};

use derive_more::From;
use directories::ProjectDirs;
use icu_collator::{Collator, CollatorPreferences, preferences::CollationNumericOrdering};
use itertools::Itertools;

use crate::error::Result;

pub static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com.xarvex", "", "yt-dli").expect("failure registering project directories")
});

// TODO: Find a "better" and generic way to handle this.
#[derive(From)]
pub enum IntoOsStringIter {
    VecString(Vec<String>),
    VecOsString(Vec<OsString>),
}

impl IntoIterator for IntoOsStringIter {
    type Item = OsString;
    type IntoIter = Box<dyn Iterator<Item = OsString>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            IntoOsStringIter::VecString(vec) => Box::new(vec.into_iter().map(OsString::from)),
            IntoOsStringIter::VecOsString(vec) => Box::new(vec.into_iter()),
        }
    }
}

pub fn sort_dir_results<I: IntoIterator<Item = std::io::Result<DirEntry>>>(
    profiles: I,
) -> Result<Vec<String>> {
    let collator_prefs = {
        let mut prefs = CollatorPreferences::default();
        prefs.numeric_ordering = Some(CollationNumericOrdering::True);
        prefs
    };
    let collator = Collator::try_new(collator_prefs, Default::default())?;

    Ok(profiles
        .into_iter()
        .filter_map(|d| d.ok())
        .map(|d| d.file_name().display().to_string())
        .sorted_by(|a, b| collator.compare(a, b))
        .collect()) // After sorting, is zero-cost.
}
