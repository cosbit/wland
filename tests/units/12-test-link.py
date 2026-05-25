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


def test_link_help_page() -> None:
    result = run_cli("link", "--help")
    output = (result.stdout + result.stderr).lower()

    assert result.returncode == 0, output
    assert "link exposes the current linux interface table" in output, output


def test_link_show_output_shape() -> None:
    result = run_cli("link", "show")
    output = result.stdout + result.stderr

    assert result.returncode == 0, output
    assert "LINKS" in output, output
    assert "lo" in output or "link/loopback" in output, output
    assert "ifindex:" in output, output
    assert "kind:" in output, output
