# Agent directives

## Workflow priority

- Prefer `nix run .#compile` for compile-only checks.
- Prefer `nix run .#build` for build validation when the app exists.
- Prefer `nix run .#test` for test validation.
- Do not call `cargo`, `pytest`, or other tool binaries directly when a matching `nix run .#...` app exists.
- If a required `nix run .#<name>` app does not exist for the task, add the missing app in `flake.nix` before relying on a direct command or wrapper script.

## Script policy

- Add a helper script only when a stable human-facing entry point is useful.
- Keep helper scripts thin wrappers around `nix run .#...` apps.
- Avoid duplicating build or test logic inside scripts.

## Validation order

- Use `nix run .#compile` before `nix run .#test` when checking code changes.
- If a workflow needs a dedicated command, create the Nix app first, then add a script alias only if needed.
- When adding new tracked source files for Nix builds or tests, run `git add .` so the file is included in the staged tree before invoking `nix run`.
