use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tokio::runtime::Builder;

#[derive(Debug, Args)]
#[command(
    about = "Manage radio hardware identity and capability intent.",
    long_about = "PHY describes the physical radio identity, country code, channel, channel width, transmit power, and hardware capability observation.",
    arg_required_else_help = true
)]
pub struct PhyCli {
    #[command(subcommand)]
    command: PhyCommand,
}

#[derive(Debug, Subcommand)]
enum PhyCommand {
    #[command(about = "Show the current PHY snapshot.")]
    Show { phy_name: String },
}

impl PhyCli {
    pub fn run(self) -> Result<()> {
        match self.command {
            PhyCommand::Show { phy_name } => {
                let runtime = Builder::new_current_thread().enable_all().build().context("failed to create runtime")?;
                let state = runtime.block_on(crate::phy::get_wireless_phy(&phy_name))?;
                print_state(&state);
                Ok(())
            }
        }
    }
}

fn print_state(state: &crate::phy::WirelessPhyState) {
    println!("PHY {}", state.identity.phy_name);
    println!("  identity:");
    println!("    phy_name:    {}", state.identity.phy_name);
    println!("    wiphy_index: {}", state.identity.wiphy_index);
    println!("    path:        {}", state.identity.path.as_deref().unwrap_or("none"));
    println!("    driver:      {}", state.identity.driver.as_deref().unwrap_or("none"));
    println!("    mac:         {}", state.identity.mac.as_deref().unwrap_or("none"));
    println!("    bus_path:    {}", state.identity.bus_path.as_deref().unwrap_or("none"));
    println!("  capabilities:");
    println!("    bands:       {:?}", state.capabilities.supported_bands);
    println!("    modes:       {:?}", state.capabilities.supported_modes);
    println!("    channels:    {}", state.capabilities.supported_channels.len());
    println!("    supports_ap: {}", state.capabilities.supports_ap);
}
