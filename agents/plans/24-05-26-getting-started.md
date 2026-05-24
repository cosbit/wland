# Initial wiring for binary compilation and basic CLI usage

## Sources of truth

- General spec: `@agents/spec/01-mvp-spec.md`
- Contract spec: `@agents/spec/02-mvp-contracts.md`
- Structure spec: `@agents/spec/03-mvp-project.md`
- CLI spec: `@agents/spec/04-mvp-cli.md`

## Goal

Get the `wland` binary compiling and expose the first CLI entry points for the core domains so the project can print usable help output before any real networking logic exists.

## Current shape
Some files exist but they are all empty.
- `src/main.rs` is the binary entry point.
- `src/wland/cli.rs` and `src/wland/http.rs` already exist as scaffolding.
- `src/wland/schema.rs` and `src/wan/schema.rs` are present as early model stubs.
- `src/dbus/server.rs` and `src/dbus/client.rs` exist as placeholders for future IPC work.

## Scope

- Wire `main.rs` to the root `wland` CLI module.
- Add root-level CLI dispatch for the top-level binary.
- Add domain-level CLI modules for:
  - `wan`
  - `lan`
  - `phy`
  - `bss`
  - `wlan`
- Ensure `wland --help` works.
- Ensure `wland <domain> --help` works for each core domain.
- Keep command handling thin and descriptive for now.
- Wire flake.nix to create a series of commands for compilation and running. nix run .#compile (compiles only); nix run .#test (compiles and tests).
- Simple pytest 

## Not in scope

- No domain command execution beyond help/placeholder output.
- No backend apply path.
- No config persistence.
- No validation or hardware probing.
- No IPC behavior yet.
- No network mutations yet.

## Desired outcome

The binary should be runnable as `wland`, and the following invocations should produce a help summary instead of failing:

- `wland --help`
- `wland wan --help`
- `wland lan --help`
- `wland phy --help`
- `wland bss --help`
- `wland wlan --help`

## Command design

- The root command should describe WLANd as the aggregate controller for desired state, observed state, runtime state, and apply operations.
- Each domain command should print a short domain-specific description.
- The CLI should be structured so future subcommands can be added without breaking the top-level shape.
- Unknown commands should fail cleanly with a usable error message.

## Execution plan

1. Confirm the binary entry point and root CLI module are wired together.
2. Define the root command parser and route domain subcommands through it.
3. Add minimal per-domain CLI modules that can render `--help` output.
4. Keep the help text aligned with the domain responsibilities described in the MVP specs.
5. Add a small compile or smoke test that covers the binary entry point and CLI dispatch.

## Acceptance criteria

- `wland` compiles as an executable binary.
- `wland --help` prints a root-level description.
- `wland wan --help`, `wland lan --help`, `wland phy --help`, `wland bss --help`, and `wland wlan --help` all return help text.
- The CLI structure is ready for later expansion into create/show/apply workflows.
- The implementation stays thin and does not introduce backend logic prematurely.

## Implementation notes

- Prefer a small and explicit CLI tree over a clever parser abstraction for the first pass.
- Keep the help strings domain-oriented and consistent with the project specs.
- Reuse the existing `wland` module scaffolding instead of duplicating entry points elsewhere.
- If the repo already has partial CLI files, extend them rather than replacing them with a new pattern.
