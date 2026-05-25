from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def rust_test(*args: str) -> subprocess.CompletedProcess[str]:
    binary = Path("/home/cosweb/Projects/wland/target/debug/wland")
    return subprocess.run(
        [str(binary), *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )


def test_phy_helpers_are_present() -> None:
    source = (ROOT / "src" / "phy" / "mod.rs").read_text(encoding="utf-8")

    assert "pub async fn get_wireless_phy" in source
    assert "pub fn parse_phy_name" in source
    assert "pub fn map_frequency_to_band" in source
    assert "pub fn map_interface_mode" in source
    assert "pub fn channel_capability" in source
    assert "pub fn derive_capability_summary" in source


def test_phy_name_examples() -> None:
    source = (ROOT / "src" / "phy" / "mod.rs").read_text(encoding="utf-8")

    assert 'assert_eq!(parse_phy_name("phy0").unwrap(), 0);' in source
    assert 'assert!(parse_phy_name("wiphy0").is_err());' in source


def test_band_and_mode_examples() -> None:
    source = (ROOT / "src" / "phy" / "mod.rs").read_text(encoding="utf-8")

    assert 'assert_eq!(map_frequency_to_band(2412), Some(WifiBand::Ghz2));' in source
    assert 'assert_eq!(map_interface_mode("Ap"), Some(InterfaceMode::Ap));' in source
