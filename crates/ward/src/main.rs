//! `ward` — run a language server under a capability profile.
//!
//! Nothing is confined yet. This is the entry point the confinement will hang
//! off, and enough of a workspace for the merge gate to have something real to
//! check. The capability profile format is the design decision everything else
//! depends on; see the README for where that stands.

use std::env;
use std::process::ExitCode;

const USAGE: &str = "\
ward — run a language server under a capability profile

usage:
    ward --version
    ward --help

No subcommands yet: the capability profile format is not settled, and a CLI
that outlives its design is harder to change than one that does not exist.";

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            // 2 rather than 1: a usage error is not a failed run.
            ExitCode::from(2)
        }
    }
}

/// Interprets the command line, returning what to print on success or the
/// message to print on stderr when the arguments make no sense.
fn run<I>(mut args: I) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    match args.next().as_deref() {
        Some("--version" | "-V") => Ok(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        None | Some("--help" | "-h") => Ok(USAGE.to_owned()),
        Some(other) => Err(format!("ward: unrecognised argument `{other}`\n\n{USAGE}")),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    fn run_with(args: &[&str]) -> Result<String, String> {
        run(args.iter().map(|s| (*s).to_owned()))
    }

    #[test]
    fn version_reports_the_package_version() {
        let out = run_with(&["--version"]).expect("--version should succeed");
        assert_eq!(out, format!("ward {}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn no_arguments_and_help_both_print_usage() {
        assert_eq!(run_with(&[]), Ok(super::USAGE.to_owned()));
        assert_eq!(run_with(&["--help"]), Ok(super::USAGE.to_owned()));
    }

    #[test]
    fn unknown_argument_is_an_error_and_shows_usage() {
        let err = run_with(&["--sandbox-everything"]).expect_err("should be rejected");
        assert!(err.contains("--sandbox-everything"), "names the argument");
        assert!(err.contains("usage:"), "shows usage");
    }
}
