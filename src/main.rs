mod bss;
mod clients;
mod common;
mod dbus;
mod dhcp;
mod firewall;
mod lan;
mod logs;
mod phy;
mod rtnet;
mod wan;
mod wlan;
mod wland;

fn main() -> anyhow::Result<()> {
    wland::cli::run()
}
