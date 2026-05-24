use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Manage logical SSID, security, and BSS policy.",
    long_about = "BSS describes the advertised network: SSID, security mode, secret reference, and client policy.",
    arg_required_else_help = true
)]
pub struct BssCli {}

impl BssCli {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
