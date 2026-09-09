---
title: Development
description: Build, test, and regenerate documentation for nu_plugin_net.
---

## Rust checks

From the repository root, run the same checks used by continuous integration:

```sh
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo build --locked
```

The Rust crate currently requires Rust `1.95.0` or newer because that is the
minimum supported by Nushell `0.115.x`.

## Work on the documentation site

The Starlight site lives in `rustdoc-net/` and uses the checked-in
`package-lock.json`:

```sh
cd rustdoc-net
npm ci
npm run dev
```

Use `npm run build` to create the production site in `rustdoc-net/dist/`.
The build first regenerates the Rust API reference from the repository's
current source, then builds the Starlight pages around it.

## Rust API generation

To regenerate only the embedded Rustdoc files:

```sh
cd rustdoc-net
npm run rustdoc
```

The generated files are placed in `rustdoc-net/public/rustdoc/` and should not
be edited by hand. Update Rust comments in `src/` and rerun the generator when
the API reference needs better context.
