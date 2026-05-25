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
    assert "ipv4:" in output.lower(), output
    assert "address:" in output.lower(), output
    assert "prefix:" in output.lower(), output
    assert "network:" in output.lower(), output
