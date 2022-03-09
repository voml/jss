#![doc = include_str!("../Readme.md")]

use std::path::PathBuf;

use clap::Parser;

pub use self::commands::JssCommands;
use jss_core::Result;

mod commands;
mod run;

/// The main application.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct JssApplication {
    /// Sets a custom config file
    #[clap(parse(from_os_str), value_name = "DIR")]
    workspace: Option<PathBuf>,
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,
    #[clap(short, long, value_name = "GLOB")]
    pattern: Option<String>,
    #[clap(short, long)]
    minify: Option<bool>,
    #[clap(long)]
    obfuscate: Option<bool>,
    #[clap(long)]
    dry_run: bool,
    #[clap(short, parse(from_occurrences))]
    details: usize,
    #[clap(long, arg_enum)]
    mode: Option<Mode>,
    #[clap(subcommand)]
    command: Option<JssCommands>,
}

fn main() -> Result<()> {
    let cli = JssApplication::parse();
    let cfg = cli.build_config();
    match cli.command {
        Some(cmd) => {
            cmd.run(&mut cfg)?;
        }
        None => {
            let cmd = JssCommands::default();

            cmd.run(&mut cfg)?;
        }
    }
    Ok(())
}
