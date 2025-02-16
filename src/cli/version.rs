use crate::version;
use crate::Result;

#[derive(Debug, clap::Args)]
pub struct Version {}

impl Version {
    pub async fn run(&self) -> Result<()> {
        println!("{}", version::version());
        Ok(())
    }
}
