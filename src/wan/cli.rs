use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Manage WAN connectivity and upstream route intent.",
    long_about = "WAN describes the upstream interface, IP assignment method, and default-route observation for the router edge.",
    arg_required_else_help = true
)]
pub struct WanCli {}

impl WanCli {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
