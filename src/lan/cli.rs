use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Manage LAN subnet, router address, and local service intent.",
    long_about = "LAN describes the local bridge or subnet, the router address, and the intent for DHCP and DNS on the inside network.",
    arg_required_else_help = true
)]
pub struct LanCli {}

impl LanCli {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
