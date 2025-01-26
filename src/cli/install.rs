use std::path::PathBuf;

use crate::Result;

/// Sets up git hooks to run angler
#[derive(Debug, clap::Args)]
#[clap()]
pub struct Install {}

impl Install {
    pub async fn run(&self) -> Result<()> {
        let hooks = PathBuf::from(".git/hooks");
        let hook_file = hooks.join("pre-commit");
        let hook_content = r#"#!/bin/sh
angler pre-commit "$@"
"#;
        xx::file::write(hook_file, hook_content)?;
        Ok(())
    }
}