use std::path::PathBuf;

use clap::Subcommand;

use crate::JssApplication;
use jss_core::Result;

#[derive(Subcommand)]
pub enum JssCommands {
    /// Create jss from json and yaml schemas.
    ///
    /// Will generate files with the same name but different extensions.
    Init {
        #[clap(parse(from_os_str), value_name = "DIR")]
        workspace: Option<PathBuf>,
        overwrite: bool,
    },
    /// Generate json schema from jss schema.
    ///
    /// Will generate files with the same name but different extensions.
    Gen {
        #[clap(parse(from_os_str), value_name = "DIR")]
        workspace: Option<PathBuf>,
        overwrite: bool,
    },
    Infer {
        #[clap(parse(from_os_str), value_name = "FILE")]
        file: Option<PathBuf>,
    },
    Mock {},
}

pub struct JssConfig {}

impl Default for JssConfig {
    fn default() -> Self {
        Self {}
    }
}

impl JssApplication {
    pub fn build_config(&self) -> JssConfig {
        JssConfig::default()
    }
}

impl Default for JssCommands {
    fn default() -> Self {
        JssCommands::Init { workspace: None, overwrite: false }
    }
}

impl JssCommands {
    pub fn run(&self, config: &mut JssConfig) -> Result<()> {
        let _ = config;
        println!("?");
        match self {
            JssCommands::Init { .. } => {}
            JssCommands::Gen { .. } => {}
            JssCommands::Infer { .. } => {}
            JssCommands::Mock { .. } => {}
        }
        Ok(())
    }
}
