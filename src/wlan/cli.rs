use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Bind a PHY to a BSS and a LAN for concrete AP wiring.",
    long_about = "WLAN describes the concrete access-point binding between a PHY, a BSS, a LAN, and the Linux interface that carries them.",
    arg_required_else_help = true
)]
pub struct WlanCli {}

impl WlanCli {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
