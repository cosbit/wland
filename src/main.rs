mod bss;
mod clients;
mod common;
mod dbus;
mod dhcp;
mod firewall;
mod lan;
mod link;
mod logs;
mod mgmt;
mod phy;
mod wan;
mod wlan;
mod wland;

fn main() -> anyhow::Result<()> {
    wland::cli::run()
}
