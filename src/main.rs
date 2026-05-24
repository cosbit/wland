mod bss;
mod lan;
mod phy;
mod wan;
mod wlan;
mod wland;

fn main() -> anyhow::Result<()> {
    wland::cli::run()
}
