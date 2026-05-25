# Agent testing directive

## Goal

Use the Nix apps exposed by `flake.nix` as the preferred entry points for compile, build, and test workflows.

## Required command order

- `nix run .#compile` for compile-only validation.
- `nix run .#build` for build validation when a dedicated build app exists.
- `nix run .#test` for test validation.

## Rules

- Prefer `nix run .#...` over invoking `cargo`, `pytest`, or other tool binaries directly.
- If a required `nix run .#<name>` app does not exist for the desired workflow, create the missing app in `flake.nix` or the matching shell script before relying on it.
- Keep new helper scripts thin wrappers around the Nix apps; do not duplicate build logic in scripts.
- Use `nix run .#compile` before `nix run .#test` when validating code changes.
- If a build or test step needs a specialized command, add a dedicated Nix app for it first.

## Expectations for agents

- Confirm the relevant app exists before using it.
- Add missing app definitions to `flake.nix` when a workflow is needed.
- Add a small script only when the workflow needs a stable human-facing entry point beyond the Nix app.
