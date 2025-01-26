#[macro_use]
extern crate log;

mod logger;
mod cli;
mod env;
mod ui;

use tokio::signal;
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
pub use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    #[cfg(unix)]
    handle_epipe();
    cli::run().await
}

#[cfg(unix)]
fn handle_epipe() {
    let mut pipe_stream = signal::unix::signal(SignalKind::pipe()).unwrap();
    tokio::spawn(async move {
        pipe_stream.recv().await;
        debug!("received SIGPIPE");
    });
}