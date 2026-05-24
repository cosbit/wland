use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Manage radio hardware identity and capability intent.",
    long_about = "PHY describes the physical radio identity, country code, channel, channel width, transmit power, and hardware capability observation.",
    arg_required_else_help = true
)]
pub struct PhyCli {}

impl PhyCli {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
