use crate::Result;

/// Sets up git hooks to run angler
#[derive(Debug, clap::Args)]
#[clap()]
pub struct PreCommit {}

impl PreCommit {
    pub async fn run(&self) -> Result<()> {
        println!("pre-commit");
        Ok(())
    }
}