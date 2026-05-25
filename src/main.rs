mod bss;
mod clients;
mod common;
mod dbus;
mod dhcp;
mod firewall;
mod lan;
mod logs;
mod mgmt;
mod phy;
mod link;
mod wan;
mod wlan;
mod wland;

fn main() -> anyhow::Result<()> {
    wland::cli::run()
}
