use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tokio::runtime::Builder;

#[derive(Debug, Args)]
#[command(
    about = "Inspect runtime network interfaces from rtnetlink.",
    long_about = "Link exposes the current Linux interface table as observed through rtnetlink.",
    arg_required_else_help = true
)]
pub struct LinkCli {
    #[command(subcommand)]
    command: LinkCommand,
}

#[derive(Debug, Subcommand)]
enum LinkCommand {
    #[command(about = "Show the current runtime interface snapshot.")]
    Show,
}

impl LinkCli {
    pub fn run(self) -> Result<()> {
        match self.command {
            LinkCommand::Show => {
                let runtime = Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .context("failed to create runtime")?;
                let state = runtime.block_on(async {
                    let service = crate::link::RtnetlinkService::new().await?;
                    service
                        .fetch(crate::link::schema::FetchRequest)
                        .await
                        .map(|result| result.value)
                })?;
                print_state(&state);
                Ok(())
            }
        }
    }
}

fn print_state(state: &crate::link::schema::RtnetState) {
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
        if let Some(ipv4) = &iface.ipv4 {
            println!("    ipv4:");
            println!("      address:   {}", ipv4.address);
            println!("      prefix:    {}", ipv4.prefix);
            println!("      network:   {}", ipv4.network);
        }
        println!(
            "    master:      {}",
            iface
                .master
                .map_or_else(|| "none".to_string(), |value| value.to_string())
        );
    }
}

fn format_kind(kind: crate::link::schema::NetdevKind) -> &'static str {
    match kind {
        crate::link::schema::NetdevKind::Bridge => "bridge",
        crate::link::schema::NetdevKind::Ethernet => "ethernet",
        crate::link::schema::NetdevKind::Wlan => "wlan",
        crate::link::schema::NetdevKind::Unknown => "unknown",
    }
}

fn format_oper(state: crate::link::schema::OperState) -> &'static str {
    match state {
        crate::link::schema::OperState::Unknown => "unknown",
        crate::link::schema::OperState::NotPresent => "not_present",
        crate::link::schema::OperState::Down => "down",
        crate::link::schema::OperState::LowerLayerDown => "lower_layer_down",
        crate::link::schema::OperState::Testing => "testing",
        crate::link::schema::OperState::Dormant => "dormant",
        crate::link::schema::OperState::Up => "up",
    }
}

fn format_bool(value: bool) -> &'static str {
    if value {
        "up"
    } else {
        "down"
    }
}
