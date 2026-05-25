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

## Common Schemas

- Put reusable value objects in `src/common/schemas.rs` only when they are shared vocabulary across domains.
- Keep domain-owned intent, runtime state, policy, and inventory types in their domain `schema.rs` files.
- Prefer small, boring shared types such as IDs, names, timestamps, address primitives, enums, and generic status wrappers.
- Do not move `Desired`, `Observed`, runtime, station/client, lease, DHCP, DNS, NAT, firewall zone, or management policy structs into `common`.
- If a common type starts growing domain-specific fields, move that specialization back into the owning domain.

## Tests Guideliness

- Use `pytest` to make CLI tests.
- Prefer CLI tests that execute the installed `wland` binary through the shared `run_cli` helper pattern used in `tests/units/11-run-cli.py` and `tests/units/12-test-link.py`.
- Use `nix run .#test` to call it.
- New unit files are sequentially numbered and named `Mm-short-desc.py`, where `M` is the major domain like `link`, `wlan`, or `wan`, and `m` is the minor field like `cli`, `http`, or `daemon`.
