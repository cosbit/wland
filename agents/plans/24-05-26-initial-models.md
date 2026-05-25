## Scaffolding initial structs based on MVP specs

## Sources of truth

- General spec: `@agents/spec/01-mvp-spec.md`
- Contract spec: `@agents/spec/02-mvp-contracts.md`
- Structure spec: `@agents/spec/03-mcp-project.md`

## Goal

Create the initial Rust data model scaffolding for WLANd so the project has a clean separation between desired state, observed state, and runtime/client state before any backend behavior is implemented.

## Scope

- Define root aggregate model types in `src/wland/schema.rs`.
- Add domain modules for the core MVP surfaces:
  - `wan`
  - `lan`
  - `phy`
  - `bss`
  - `wlan`
  - `firewall`
  - `dhcp`
  - `clients`
  - `logs`
  - `dbus`
- Add the minimum module wiring needed for the crate to compile cleanly with the new model split.
- Add placeholder API/CLI module entry points only where the current tree already expects them.
- Add tests that validate the project compiles and that the top-level model types are present.
- Add `src/common/exceptions.rs` and `src/common/errcodes.rs`

## Not in scope

- No backend rendering yet.
- No netlink, hostapd, nftables, or dnsmasq integration yet.
- No validation logic beyond structural type definitions.
- No persistence, migration, or transaction handling yet.
- No runtime observation logic yet.

## Model boundaries

- `wland` owns the aggregate root and composes all domain modules.
- `wan` models upstream interface behavior, addressing, NAT intent, and firewall profile references.
- `lan` models internal bridges, subnets, DHCP intent, DNS intent, and router address data.
- `phy` models physical radio identity and capability metadata.
- `bss` models SSID, security, secret references, and client policy.
- `wlan` models the concrete binding between `phy`, `bss`, and `lan`.
- `firewall` models zone policy, NAT intent, and explicit forwarding policy.
- `dhcp` models DHCP server intent and lease-related state.
- `clients` models associated stations and lease-derived runtime client data.
- `logs` models validation/apply/service log surfaces.
- `dbus` owns the IPC surface and should stay thin.

## Tests
`12-validate-models.py`
- test_

## Execution plan

1. Normalize the root module layout so `src/wland/schema.rs` exposes the top-level desired, observed, and runtime containers.
2. Add domain schema files for each MVP area with only the fields required by the specs.
3. Add module boundaries and `mod.rs` files so each domain can grow independently without bloating the aggregate root.
4. Wire the existing `wan` and `wland` code into the new structure before expanding the remaining domains.
5. Add compile-focused tests that prevent the aggregate schema from drifting or being left incomplete.

## Acceptance criteria

- The root model cleanly represents:
  - desired state
  - observed state
  - runtime/client state
  - validation/apply metadata where appropriate
- Each MVP domain has a dedicated module boundary instead of one monolithic schema file.
- The initial type definitions match the contract spec vocabulary closely enough to support later backend work.
- The tree compiles with the new module layout.
- A basic test covers the model surface or at minimum exercises compilation of the aggregate schema.

## Implementation notes

- Prefer explicit, domain-focused structs over overly generic maps for the first pass.
- Keep fields close to the spec language so later validation and rendering can map cleanly onto the models.
- If a field is not needed to support the MVP contracts yet, leave it out rather than guessing.
- Preserve compatibility with the existing untracked scaffold files in the working tree.
