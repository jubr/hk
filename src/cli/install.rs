use std::path::PathBuf;

use crate::{config::Config, Result};

/// Sets up git hooks to run hk
#[derive(Debug, clap::Args)]
#[clap()]
pub struct Install {}

impl Install {
    pub async fn run(&self) -> Result<()> {
        let config = Config::get()?;
        let hooks = PathBuf::from(".git/hooks");
        let add_hook = |hook: &str| {
            let hook_file = hooks.join(hook);
            let hook_content = format!(
                r#"#!/bin/sh
hk run {hook} "$@"
"#
            );
            xx::file::write(&hook_file, &hook_content)?;
            xx::file::make_executable(&hook_file)?;
            println!("Installed hk hook: .git/hooks/{hook}");
            Result::<(), miette::Report>::Ok(())
        };
        if config.pre_commit.is_some() {
            add_hook("pre-commit")?;
        }
        if config.pre_push.is_some() {
            add_hook("pre-push")?;
        }
        Ok(())
    }
}
