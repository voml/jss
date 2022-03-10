use std::env::{current_dir, set_current_dir};

use crate::JssApplication;
use jss_core::Result;

impl JssApplication {
    fn set_workspace(&self) -> Result<()> {
        if let Some(s) = &self.workspace {
            set_current_dir(s)?;
        }
        println!("Current workspace: {:?}", current_dir()?);
        Ok(())
    }
}

impl JssApplication {}
