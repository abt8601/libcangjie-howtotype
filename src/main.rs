use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use itertools::Itertools;

use libcangjie_howtotype::{CangjieCode, LibCangjieHowToType, NewError};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The character to query.
    character: String,
    /// The version of Cangjie used.
    #[arg(
        short = 'C',
        long,
        value_name = "VERSION",
        default_value_t = CangjieVersion::V3,
        value_enum,
    )]
    cj_version: CangjieVersion,
    /// The output format.
    #[arg(short, long, default_value_t = Format::Radical, value_enum)]
    format: Format,
    /// The separator between codes.
    #[arg(short, long, default_value_t = String::from("\n"))]
    separator: String,
    /// Do not report an error when the command doesn't know how to type.
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, ValueEnum)]
enum CangjieVersion {
    #[value(name = "3")]
    V3,
    #[value(name = "5")]
    V5,
}

impl From<CangjieVersion> for libcangjie_howtotype::CangjieVersion {
    fn from(value: CangjieVersion) -> Self {
        match value {
            CangjieVersion::V3 => Self::V3,
            CangjieVersion::V5 => Self::V5,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, ValueEnum)]
enum Format {
    #[value(alias = "c")]
    Code,
    #[value(alias = "r")]
    Radical,
}

fn main() -> ExitCode {
    human_panic::setup_panic!();

    // Reference:
    // https://github.com/crate-ci/typos/blob/master/crates/typos-cli/src/bin/typos-cli/main.rs#L21-L31
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) if e.use_stderr() => {
            let _ = e.print();
            return ExitCode::from(u8::try_from(exitcode::USAGE).expect("Invalid exit code"));
        }
        Err(e) => {
            let _ = e.print();
            return ExitCode::SUCCESS;
        }
    };

    let cangjie = match LibCangjieHowToType::new() {
        Ok(cangjie) => cangjie,
        Err(NewError::DBError(rusqlite::Error::SqliteFailure(e, _)))
            if matches!(
                e.code,
                rusqlite::ffi::ErrorCode::SystemIoFailure
                    | rusqlite::ffi::ErrorCode::DatabaseCorrupt
                    | rusqlite::ffi::ErrorCode::CannotOpen
                    | rusqlite::ffi::ErrorCode::NotADatabase,
            ) =>
        {
            let exit_code = if matches!(e.code, rusqlite::ffi::ErrorCode::SystemIoFailure) {
                exitcode::IOERR
            } else {
                exitcode::OSFILE
            };

            eprintln!("Error: Cannot open libcangjie's database: {e}");
            return ExitCode::from(u8::try_from(exit_code).expect("Invalid exit code"));
        }
        Err(e) => panic!("`LibCangjieHowToType::new` failed: {e}"),
    };

    let how_to_type = cangjie
        .how_to_type(&args.character, args.cj_version.into())
        .expect("`LibCangjieHowToType::how_to_type` failed");

    if how_to_type.is_empty() {
        if args.quiet {
            return ExitCode::SUCCESS;
        } else {
            eprintln!("Error: Don't know how to type '{}'", args.character);
            return ExitCode::FAILURE;
        }
    }

    match args.format {
        Format::Code => println!(
            "{}",
            how_to_type
                .iter()
                .map(CangjieCode::codes)
                .format(&args.separator),
        ),
        Format::Radical => println!(
            "{}",
            how_to_type
                .iter()
                .map(CangjieCode::radicals)
                .format(&args.separator),
        ),
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
