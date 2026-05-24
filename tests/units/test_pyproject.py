from pathlib import Path


def test_python_project_metadata_exists() -> None:
    root = Path(__file__).resolve().parents[2]

    assert (root / "pyproject.toml").is_file()
    assert (root / ".python-version").read_text(encoding="utf-8").strip() == "3.13"
