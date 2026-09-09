---
title: Use `net`
description: Query and transform network-interface data in Nushell.
---

## Run the command

After the plugin is installed and loaded, invoke `net` with no arguments:

```nu
net
```

The result is a Nushell table. Each row describes one interface as it exists at
the time of the call.

## Filter interfaces

Nushell's structured-data commands work directly on the result:

```nu
# Interfaces that are up.
net | where flags.is_up

# The loopback interface.
net | where name == 'lo'

# Interfaces with at least one address.
net | where ($it.ips | is-not-empty)
```

Interface names and flag values are platform-dependent, so do not assume that
every host has the same set of rows.

## Inspect addresses

The `ips` field is itself a table:

```nu
net | each {|interface|
  {
    name: $interface.name,
    addresses: ($interface.ips | get addr),
  }
}
```

Each address record contains `addr`, `type` (`v4` or `v6`), and `prefix`. The
prefix is the network prefix length reported by the operating system.

## Flatten flags for reporting

The five common flags can be promoted to top-level columns when that is more
convenient for a table or export:

```nu
net | flatten flags | select name if_index is_up is_loopback is_multicast
```

The command reports interface metadata only. It does not measure traffic,
inspect routing tables, or open network connections.
