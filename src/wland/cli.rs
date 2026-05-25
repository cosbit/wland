use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "wland",
    version,
    about = "WLANd is the aggregate controller for desired state, observed state, runtime state, and apply operations.",
    long_about = "WLANd is the aggregate controller for desired state, observed state, runtime state, and apply operations.\n\nIt keeps the domain split visible at the top level while the backend-specific implementation is still being built.",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Manage WAN connectivity and upstream route intent.")]
    Wan(crate::wan::cli::WanCli),
    #[command(about = "Manage LAN subnet, router address, and local service intent.")]
    Lan(crate::lan::cli::LanCli),
    #[command(about = "Manage radio hardware identity and capability intent.")]
    Phy(crate::phy::cli::PhyCli),
    #[command(about = "Manage logical SSID, security, and BSS policy.")]
    Bss(crate::bss::cli::BssCli),
    #[command(about = "Bind a PHY to a BSS and a LAN for concrete AP wiring.")]
    Wlan(crate::wlan::cli::WlanCli),
    #[command(about = "Inspect runtime network interfaces from rtnetlink.")]
    Link(crate::link::cli::LinkCli),
}


pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Wan(cmd)) => cmd.run(),
        Some(Command::Lan(cmd)) => cmd.run(),
        Some(Command::Phy(cmd)) => cmd.run(),
        Some(Command::Bss(cmd)) => cmd.run(),
        Some(Command::Wlan(cmd)) => cmd.run(),
        Some(Command::Link(cmd)) => cmd.run(),
        None => {
            Cli::command().print_long_help()?;
            println!();
            Ok(())
        }
    }
}
