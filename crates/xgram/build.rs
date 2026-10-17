use cargo_emit::warning;
use rustc_version::{Channel, VersionMeta, version_meta};

const TESTED_NIGHTLY_DATE: &str = "2026-09-18"; // Date set to one day prior from the one in `rust-toolchain.toml`, because of how Rust nightly builds work.

fn main() {
    let VersionMeta {
        channel,
        commit_date,
        ..
    } = version_meta().unwrap();
    if matches!(channel, Channel::Nightly)
        && let Some(commit_date) = commit_date
        && commit_date != TESTED_NIGHTLY_DATE
    {
        warning!(
            "Current compiler nightly version date ({}) does not match the version crate was \
             tested with ({}). You might be good, but if you encounter any errors - use the \
             tested version.",
            commit_date,
            TESTED_NIGHTLY_DATE
        );
    }
}
