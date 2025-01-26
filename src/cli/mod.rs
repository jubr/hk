use crate::Result;
use clap::Parser;

mod install;
mod pre_commit;

#[derive(Debug, clap::Parser)]
#[clap(name = "angler", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Install(install::Install),
    PreCommit(pre_commit::PreCommit),
}

pub async fn run() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Install(install) => install.run().await,
        Commands::PreCommit(pre_commit) => pre_commit.run().await,
    }
}