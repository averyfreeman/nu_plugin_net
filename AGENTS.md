# nu_plugin_net maintenance plan

## Objective

Maintain compatibility with Nushell `nu@~0.115.x` and release the update as
`nu_plugin_net` `1.12.0` while reviewing the plugin for bugs, type-safety gaps,
anti-patterns, and missing regression coverage.

## Required implementation

- Keep `nu-plugin` and `nu-protocol` on the same `~0.115.0` minor line.
- Set the crate's minimum Rust version to `1.95.0`, matching Nushell `0.115.x`.
- Regenerate `Cargo.lock` with Cargo; do not hand-edit dependency checksums.
- Keep the public command name (`net`) and existing output field names stable.
- Ensure the declared output type matches runtime values, including `nothing`
  for interfaces without a MAC address.
- Use the lossless `u32`-to-`i64` conversion when exposing the platform
  interface index as a Nushell integer.
- Keep the signature's nested IP and flag schemas aligned with emitted records.
- Leave README/changelog changes and the existing documentation publishing
  workflows out of scope for this update.
- Keep formatting, tests, Clippy, and locked builds enforced in CI.

## Tests and verification

Add deterministic unit tests for IPv4/IPv6 mapping, prefixes, flags, complete
interface records, optional MAC values, and the command's nested output type.

Run the following from the repository root:

```text
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo build --locked
```

With the installed Nushell `0.115.1`, register the built plugin in an isolated
temporary configuration and invoke `net`. Confirm that the command returns a
table/list without protocol or type errors and that rows expose `name`,
`description`, `if_index`, `mac`, `ips`, and `flags`.

## Constraints

- Preserve unrelated user changes in the working tree.
- Prefer Cargo-generated lockfile updates and existing Nushell APIs over
  compatibility shims.
- Do not modify `.github/workflows/web.yml` or other documentation-publishing
  workflow files; the documentation stack is planned for replacement.
- If crates.io is unavailable, use cached crates for offline validation and
  report any checks that could not be completed.
