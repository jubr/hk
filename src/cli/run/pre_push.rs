use crate::config::Config;
use crate::{git::Git, Result};
use std::path::Path;

/// Sets up git hooks to run hk
#[derive(Debug, clap::Args)]
#[clap(visible_alias = "pp")]
pub struct PrePush {
    /// Run on all files instead of just staged files
    #[clap(short, long)]
    all: bool,
}

impl PrePush {
    pub async fn run(&self) -> Result<()> {
        let config = Config::read(Path::new("hk.pkl"))?;
        let mut repo = Git::new()?;
        let mut result = config.run_hook("pre_push", self.all, &repo).await;

        if let Err(err) = repo.pop_stash() {
            if result.is_ok() {
                result = Err(err);
            } else {
                warn!("Failed to pop stash: {}", err);
            }
        }
        result
    }
}
