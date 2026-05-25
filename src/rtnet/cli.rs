use anyhow::{Context, Result};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(
    about = "Inspect runtime network interfaces from rtnetlink.",
    long_about = "RTNL exposes the current Linux interface table as observed through rtnetlink.",
    arg_required_else_help = true
)]
pub struct RtnetCli {
    #[command(subcommand)]
    command: RtnetCommand,
}

#[derive(Debug, Subcommand)]
enum RtnetCommand {
    #[command(about = "Show the current runtime interface snapshot.")]
    Show,
}

impl RtnetCli {
    pub fn run(self) -> Result<()> {
        match self.command {
            RtnetCommand::Show => {
                let service = tokio::runtime::Handle::current()
                    .block_on(crate::rtnet::RtnetlinkService::new())
                    .context("failed to create rtnetlink service")?;
                let state = tokio::runtime::Handle::current()
                    .block_on(service.fetch(crate::rtnet::schema::FetchRequest))?
                    .value;
                print_state(&state);
                Ok(())
            }
        }
    }
}

fn print_state(state: &crate::rtnet::schema::RtnetState) {
    println!("LINKS");
    for iface in &state.interfaces {
        println!("  {}", iface.ifname);
        println!("    ifindex:     {}", iface.ifindex);
        println!("    kind:        {}", format_kind(iface.kind));
        println!("    admin:       {}", format_bool(iface.admin_up));
        println!("    oper:        {}", format_oper(iface.oper_state));
        if let Some(mtu) = iface.mtu {
            println!("    mtu:         {}", mtu);
        }
        if let Some(mac) = &iface.mac {
            println!("    mac:         {}", mac.0);
        }
        println!("    master:      {}", iface.master.map_or_else(|| "none".to_string(), |value| value.to_string()));
    }
}

fn format_kind(kind: crate::rtnet::schema::NetdevKind) -> &'static str {
    match kind {
        crate::rtnet::schema::NetdevKind::Bridge => "bridge",
        crate::rtnet::schema::NetdevKind::Ethernet => "ethernet",
        crate::rtnet::schema::NetdevKind::Wlan => "wlan",
        crate::rtnet::schema::NetdevKind::Unknown => "unknown",
    }
}

fn format_oper(state: crate::rtnet::schema::OperState) -> &'static str {
    match state {
        crate::rtnet::schema::OperState::Unknown => "unknown",
        crate::rtnet::schema::OperState::NotPresent => "not_present",
        crate::rtnet::schema::OperState::Down => "down",
        crate::rtnet::schema::OperState::LowerLayerDown => "lower_layer_down",
        crate::rtnet::schema::OperState::Testing => "testing",
        crate::rtnet::schema::OperState::Dormant => "dormant",
        crate::rtnet::schema::OperState::Up => "up",
    }
}

fn format_bool(value: bool) -> &'static str {
    if value { "up" } else { "down" }
}
