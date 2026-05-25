import os
from pathlib import Path
import subprocess


def run_cli(*args: str) -> subprocess.CompletedProcess[str]:
    root = Path(__file__).resolve().parents[2]
    binary = Path(os.environ.get("WLAND_BIN", root / "target" / "debug" / "wland"))

    return subprocess.run(
        [str(binary), *args],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )


def test_cli_help_pages() -> None:
    cases = [
        (["--help"], "aggregate controller"),
        (["wan", "--help"], "WAN describes the upstream interface"),
        (["lan", "--help"], "LAN describes the local bridge or subnet"),
        (["phy", "--help"], "PHY describes the physical radio identity"),
        (["bss", "--help"], "BSS describes the advertised network"),
        (["wlan", "--help"], "WLAN describes the concrete access-point binding"),
    ]

    for args, expected in cases:
        result = run_cli(*args)
        output = (result.stdout + result.stderr).lower()

        assert result.returncode == 0, output
        assert expected.lower() in output, output
