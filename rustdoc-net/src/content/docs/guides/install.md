---
title: Install
description: Install and register nu_plugin_net with Nushell.
---

`nu_plugin_net` is distributed as a Rust crate and as a source repository.
After the binary is available, register it with Nushell and load its command
into the current session.

## Install from crates.io

The latest published crate can be installed with Cargo:

```sh
cargo install nu_plugin_net --locked
```

This places the executable in Cargo's binary directory, commonly
`~/.cargo/bin/nu_plugin_net`.

## Install from source

To build the current checkout:

```sh
git clone https://github.com/averyfreeman/nu_plugin_net.git
cd nu_plugin_net
cargo install --path . --locked
```

The plugin currently targets Nushell `0.115.x` and requires Rust `1.95.0` or
newer.

## Register the plugin

Run these commands in Nushell:

```nu
plugin add ~/.cargo/bin/nu_plugin_net
plugin use net
```

`plugin add` records the executable in Nushell's plugin registry. `plugin use`
loads the registered command into the current session. Restart Nushell or run
`plugin use net` again after installing a newer binary.

## Verify the installation

```nu
net | length
net | columns
```

The first command reports the number of interfaces visible to the operating
system. The second should list `name`, `description`, `if_index`, `mac`, `ips`,
and `flags`.
