---
title: Output schema
description: Field-by-field reference for the `net` command.
---

`net` accepts no input and returns a table. The field order is stable in the
current command implementation, but scripts should prefer named columns over
positional assumptions.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | `string` | Operating-system interface name, such as `lo` or `enp0s31f6`. |
| `description` | `string` | Platform-provided description; it may be empty. |
| `if_index` | `int` | Operating-system interface index. |
| `mac` | `string \| nothing` | Formatted MAC address, or `nothing` when unavailable. |
| `ips` | table | Address records assigned to the interface. |
| `flags` | record | Common interface state flags. |

## `ips` records

Each row in `ips` contains:

| Field | Type | Meaning |
| --- | --- | --- |
| `addr` | `string` | Textual IPv4 or IPv6 address. |
| `type` | `string` | `v4` for IPv4 or `v6` for IPv6. |
| `prefix` | `int` | Network prefix length, for example `24` or `128`. |

An interface can have no addresses, one address, or multiple addresses. The
list is not restricted to one address family.

## `flags` record

The `flags` record contains booleans derived from the platform's interface
flags:

| Field | Meaning |
| --- | --- |
| `is_up` | The interface is administratively up. |
| `is_broadcast` | The interface supports broadcast traffic. |
| `is_loopback` | The interface is a loopback interface. |
| `is_point_to_point` | The interface is point-to-point. |
| `is_multicast` | The interface supports multicast traffic. |

Flag meaning and available interfaces depend on the operating system and
network configuration.
