use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use execlocus::{collect_report, model::Profile, renderers};

#[derive(Debug, Parser)]
#[command(name = "execlocus")]
#[command(version, about = "See what your agent context resolves—and why.")]
struct Cli {
    #[arg(long, value_enum, default_value_t = ProfileArg::Balanced, global = true)]
    profile: ProfileArg,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ProfileArg {
    ShareFirst,
    Balanced,
    LinuxFirst,
}

impl From<ProfileArg> for Profile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::ShareFirst => Self::ShareFirst,
            ProfileArg::Balanced => Self::Balanced,
            ProfileArg::LinuxFirst => Self::LinuxFirst,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run every implemented rule and show the complete terminal report.
    Check,
    /// Produce a machine-readable report.
    Report {
        #[arg(long, value_enum, default_value_t = ReportFormat::Json)]
        format: ReportFormat,
        /// Redact identity and absolute paths before JSON serialization.
        /// Markdown reports are always redacted.
        #[arg(long)]
        redact: bool,
    },
    /// Explain one implemented rule using the current report and its evidence.
    Explain {
        /// Rule identifier, for example ENV002. Matching is case-insensitive.
        rule_id: String,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ReportFormat {
    Json,
    Markdown,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Some(Command::Explain { rule_id }) = &cli.command {
        let Some(definition) = execlocus::rules::definition(rule_id) else {
            eprintln!(
                "unknown rule ID: {rule_id}. Implemented IDs: {}",
                execlocus::rules::RULE_DEFINITIONS
                    .iter()
                    .map(|definition| definition.id)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            return ExitCode::from(2);
        };
        let report = collect_report(cli.profile.into());
        print!("{}", renderers::explain::render(&report, definition));
        return ExitCode::SUCCESS;
    }

    let report = collect_report(cli.profile.into());

    match cli.command {
        Some(Command::Report { format, redact }) => match format {
            ReportFormat::Json => {
                let redacted;
                let report = if redact {
                    redacted = execlocus::privacy::redact_for_sharing(&report);
                    &redacted
                } else {
                    &report
                };
                match renderers::json::render(report) {
                    Ok(output) => println!("{output}"),
                    Err(error) => {
                        eprintln!("failed to render JSON report: {error}");
                        return ExitCode::from(2);
                    }
                }
            }
            ReportFormat::Markdown => print!("{}", renderers::markdown::render(&report)),
        },
        Some(Command::Check) | None => print!("{}", renderers::terminal::render(&report)),
        Some(Command::Explain { .. }) => {
            unreachable!("explain is handled before report collection")
        }
    }

    if report.has_error_findings() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
